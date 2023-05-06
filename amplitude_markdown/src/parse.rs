use std::{ffi::OsStr, fs, path::Path};

use amplitude_common::config::ParseConfig;
use anyhow::{ensure, Context};

use git2::build::RepoBuilder;
use rand::Rng;
use tracing::warn;

use crate::{
    inject::{self},
    link_concat::link_concat_callback,
    state::{
        article::{parse_frontmatter, ArticleConfig},
        ParseState,
    },
};
use comrak::{parse_document_refs, Arena, ComrakOptions, RefMap};

/// Parse the input `md` and return the output `html`.
pub(crate) fn parse(
    config: &ArticleConfig,
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
                warn!("Broken link `{}` in {:?}", link, config.id);
            }
            out
        }),
    );
    // do things
    inject::inject(config, out, &this_refs, state)?;

    let mut cm = vec![];
    comrak::format_html(out, options, &mut cm).context("while parsing AST to html")?;

    Ok((String::from_utf8(cm).unwrap(), this_refs))
}

/// Clones the articles repo
pub fn clone_repo(config: &ParseConfig) -> anyhow::Result<()> {
    let clone_path = &config.clone_path;
    fs::create_dir_all(clone_path)?;

    // delete everything
    for item in fs::read_dir(clone_path)? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    RepoBuilder::new().clone(&config.git_url, Path::new(clone_path))?;

    Ok(())
}

/// assuming `config.articles.clone_path` exists, parses the articles into html
pub fn parse_dir(config: &ParseConfig) -> anyhow::Result<()> {
    let mut state = ParseState::default();
    let input = Path::new(&config.clone_path);
    let output = Path::new(&config.output_path);

    fs::create_dir_all(output)?;
    // delete everything in output
    for item in fs::read_dir(output)? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    // recursively parse input directory
    for item in fs::read_dir(input)? {
        let item = item?;
        let path = item.path();

        if path.is_dir() {
            let name = path.file_name().unwrap();
            if name.to_str().unwrap().starts_with(".") {
                continue;
            }
            
            fs::create_dir(&output.join(name))?;
            let suffix = path.strip_prefix(input).unwrap();

            if path.join("header.md").exists() {
                let (article_config, s) = parse_frontmatter(input)?;
                let (_, refs) = parse(&article_config, &s, &RefMap::new(), &mut state)
                    .with_context(|| format!("While parsing header file {:?}/header.md", path))?;

                internal_parse_dir(config, &path, &output.join(suffix), &refs, &mut state)
                    .with_context(|| format!("While parsing dir {:?}", path))?;
            } else {
                internal_parse_dir(
                    config,
                    &path,
                    &output.join(suffix),
                    &RefMap::new(),
                    &mut state,
                )
                .with_context(|| format!("While parsing dir {:?}", path))?;
            }
        }
    }

    Ok(())
}

fn internal_parse_dir<P: AsRef<Path>>(
    config: &ParseConfig,
    input: P,
    output: P,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<()> {
    let input = input.as_ref();
    let output = output.as_ref();

    for item in fs::read_dir(input)? {
        let item = item?;
        let path = item.path();

        if path.is_file() {
            let name = path.file_name().unwrap();
            match path.extension().map(|s| s.to_str().unwrap()) {
                Some("md") => {}
                _ => {
                    fs::copy(path.to_path_buf(), output.join(name))?;
                }
            }
            if name == "header.md" {
                continue;
            }

            let (article_config, s) = parse_frontmatter(&path)?;
            let (html, _) = parse(&article_config, &s, &RefMap::new(), state)
                .with_context(|| format!("While parsing file {:?}", path))?;

            ensure!(
                !state.article_ids.contains_key(&article_config.id),
                "Duplicate article id: {}",
                article_config.id
            );
            fs::write(output.join(name), html)?;
        } else if path.is_dir() {
            let name = path.file_name().unwrap();

            // ignore hidden folders
            if name.to_str().unwrap().starts_with(".") {
                continue;
            }
            
            fs::create_dir(&output.join(name))?;
            let output = output.join(path.strip_prefix(input).unwrap());

            if input.join("header.md").exists() {
                let (article_config, s) = parse_frontmatter(input)?;
                let (_, refs) = parse(&article_config, &s, &RefMap::new(), state)
                    .with_context(|| format!("While parsing header file {:?}/header.md", path))?;

                internal_parse_dir(config, &path, &output, &refs, state)
                    .with_context(|| format!("While parsing dir {:?}", path))?;
            } else {
                internal_parse_dir(config, &path, &output, &refs, state)
                    .with_context(|| format!("While parsing dir {:?}", path))?;
            }
        }
    }

    Ok(())
}
