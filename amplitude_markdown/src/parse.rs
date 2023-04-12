use std::{default::default, fs, path::Path};

use amplitude_common::config;
use anyhow::Context;

use tracing::warn;

use crate::{
    inject::{self},
    link_concat::link_concat_callback,
    state::{
        article::{parse_frontmatter, ArticleConfig},
        index::{ChildEntry, RawCourseConfig, RawDirConfig},
        ParseState,
    },
};
use comrak::{
    parse_document_refs, Arena, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions,
    ListStyleType, RefMap,
};

/// Parse the input `md` and return the output `html`.
///
/// ## Behavior
///
/// - Link concatenation is supported
pub(crate) fn parse_and_refs<P: AsRef<Path>>(
    article: P,
    input: &str,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<(String, RefMap)> {
    let mut this_refs = parse_document_refs(&Arena::new(), input);
    this_refs.extend(refs.clone());

    // were not modifying options, so we can be sneaky
    // also im just too lazy to refactor this
    let options = unsafe { &*(&state.options as *const ComrakOptions) };

    let arena = Arena::new();
    let out = comrak::parse_document_with_broken_link_callback(
        &arena,
        input,
        options,
        Some(&mut |link| {
            let out = link_concat_callback(link, &this_refs);
            if out.is_none() {
                warn!("Broken link `{}` in {:?}", link, article.as_ref());
            }
            out
        }),
    );
    // do things
    inject::inject(article.as_ref(), out, &this_refs, state)?;

    let mut cm = vec![];
    comrak::format_html(out, options, &mut cm).context("while parsing AST to html")?;

    Ok((String::from_utf8(cm).unwrap(), this_refs))
}

pub(crate) fn parse<P: AsRef<Path>>(
    article: P,
    input: &str,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<String> {
    let (out, _) = parse_and_refs(article, input, refs, state)?;
    Ok(out)
}

/// Parse all files in the input directory and all its subdirectories, and write
/// the output to the output directory. Any files included in the input will
/// also be copied over to the output directory. It will delete any files in the
/// output directory that have no counterpart in the input directory.
pub fn parse_dir<P: AsRef<Path>>(input: P, output: P) -> anyhow::Result<ParseState> {
    let options = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            tagfilter: true,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: true,
            header_ids: None,
            footnotes: true,
            description_lists: true,
            front_matter_delimiter: Some("---".to_string()),
        },
        parse: default(),
        render: ComrakRenderOptions {
            github_pre_lang: false,
            full_info_string: true,
            unsafe_: true,
            hardbreaks: false,
            width: 0,
            escape: false,
            list_style: ListStyleType::default(),
            sourcepos: false,
        },
    };

    if !output.as_ref().exists() {
        fs::create_dir_all(output.as_ref())?;
    }

    let mut state = ParseState::default();
    state.options = options;

    let children = if let Ok(s) = fs::read_to_string(input.as_ref().join("header.md")) {
        let refs = comrak::parse_document_refs(&Arena::new(), &s);
        parse_dir_internal(0, input, output, &refs, &mut state)
    } else {
        parse_dir_internal(0, input, output, &RefMap::new(), &mut state)
    }
    .context("while parsing markdown files")?;
    dbg!(children);

    Ok(state)
}

/// Returns the new refs and whether or not the index is readable
fn parse_index(
    depth: u8,
    i: &Path,
    o: &Path,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<(RefMap, bool)> {
    // parse index.md
    let mut get = |s: &str| -> anyhow::Result<(String, RefMap)> {
        let other_refs = comrak::parse_document_refs(&Arena::new(), s);
        let mut refs = refs.clone();
        refs.extend(other_refs);
        // also parse index.md to add any of the things it has
        parse_and_refs(&i, s, &refs, state)
            .with_context(|| format!("while parsing {}", i.display()))
    };

    let new_refs;
    let mut readable = false;
    match depth {
        // global index, dont parse the header dont write out
        // dont pass go dont collect $200
        0 => {
            let (_, refs) = get(&fs::read_to_string(&i)?)?;
            new_refs = refs;
        }
        // course index, parse header and write out
        1 => {
            let (cfg, s): (RawCourseConfig, String) =
                parse_frontmatter(&i).context("while parsing frontmatter")?;
            let (s, refs) = get(&s)?;

            if cfg.readable {
                readable = true;
                fs::write(o, s)?;
            }
            new_refs = refs;
        }
        // else, get dir cfg and write out
        _ => {
            let (cfg, s): (RawDirConfig, String) =
                parse_frontmatter(&i).context("while parsing frontmatter")?;
            let (s, refs) = get(&s)?;

            if cfg.readable {
                readable = true;
                fs::write(o.join("index.html"), s)?;
            }
            new_refs = refs;
        }
    }

    Ok((new_refs, readable))
}

fn parse_dir_internal<P: AsRef<Path>>(
    depth: u8,
    input: P,
    output: P,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<Vec<ChildEntry>> {
    let input = input.as_ref();
    let output = output.as_ref();

    // check for files in output that are not in input, and delete them
    for entry in fs::read_dir(output)? {
        let o = entry?.path();
        let mut i = input.join(o.strip_prefix(output).unwrap());

        match o.extension().and_then(|e| e.to_str()).unwrap_or_default() {
            // check for the md counterpart
            "html" => {
                i.set_extension("md");
                // dbg!(&i, &o);
                if !i.exists() {
                    fs::remove_file(o)?;
                }
            }
            // just get rid of it
            "md" | "toml" => fs::remove_file(o)?,
            _ => {
                // dbg!(&o);
                if !i.exists() {
                    if o.is_dir() {
                        fs::remove_dir_all(o)?;
                    } else {
                        fs::remove_file(o)?;
                    }
                }
            }
        }
    }

    let mut children = vec![];

    // go through all the files, copying them over and parsing them
    for entry in fs::read_dir(input)? {
        let i = entry?.path();
        let mut child = ChildEntry::new(i.to_path_buf());

        let o = output.join(i.clone().strip_prefix(input).unwrap());

        // if its a dir, update links and call the fn again
        if i.is_dir() {
            if !o.exists() {
                fs::create_dir(&o)?;
            }
            // top level -> course

            // parse index.md
            let index = input.join("index.md");
            let c = if index.exists() {
                let (new_refs, readable) =
                    parse_index(depth, &index, &o.join("index.html"), refs, state)
                        .with_context(|| format!("while parsing index file {}", index.display()))?;
                child.readable = readable;
                parse_dir_internal(depth + 1, &i, &o, &new_refs, state)
                    .with_context(|| format!("parsing dir {}", i.display()))?
            } else {
                parse_dir_internal(depth + 1, &i, &o, refs, state)
                    .with_context(|| format!("parsing dir {}", i.display()))?
            };
            if c.is_empty() && !child.readable {
                continue;
            }
            child.children = c;
        } else {
            let ext = i.extension().unwrap_or_default();
            let name = i.file_name().unwrap_or_default();

            match ext.to_str().unwrap_or_default() {
                "md" => {
                    // ignore index.md
                    if name == "index.md" {
                        continue;
                    }

                    // otherwise, parse
                    anyhow::ensure!(
                        depth >= 1,
                        "File: {name:?} must be in the article directory"
                    );
                    let (cfg, s): (ArticleConfig, String) =
                        parse_frontmatter(&i).with_context(|| {
                            format!("while parsing frontmatter for {}", i.display())
                        })?;

                    child.readable = true;
                    let path = i.strip_prefix(config::INPUT.clone()).unwrap();
                    let output = parse(path, &s, refs, state)
                        .with_context(|| format!("while parsing file {}", i.display()))?;
                    state.insert_article_config(path, cfg);

                    fs::write(o.with_extension("html"), output)?;
                }
                _ => {
                    // copy over the file
                    fs::copy(i, o)?;
                    continue;
                }
            }
        }
        children.push(child);
    }

    Ok(children)
}

#[cfg(test)]
mod tests {}
