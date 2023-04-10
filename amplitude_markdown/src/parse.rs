use std::{
    default::default,
    fs,
    path::{Path, PathBuf},
    sync::{self, Arc},
    thread, time,
};

use amplitude_common::{
    config,
    state::{
        config::{parse_article_config, TracksRaw},
        ParseState, State,
    },
};
use anyhow::Context;
use notify::{Config, RecommendedWatcher, Watcher};
use parking_lot::RwLock;
use tracing::{error, info, warn};

use crate::{
    inject::{self},
    link_concat::link_concat_callback,
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
    comrak::format_html(out, options, &mut cm).context("While parsing AST to html")?;

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
///
/// When encountering a file called `header.md`, it will parse it as a link
/// definition file and it will not be included in the output.
///
/// `header.md` files will add on to each other, and apply to all the files in
/// their directory and below it.
///
/// ## Notes on Behavior
///
///  - `.md` files will be parsed and converted to `.html` files
///  - `.md` files in the output directory will be removed
///  - `.toml` files in the output directory will be removed
///
/// ## Special Files
///
///  - `header.md` files will be parsed as link definition files and will not be
///    included in the output
///  - `config.toml` files will be parsed to register the course
///
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

    if let Ok(s) = fs::read_to_string(input.as_ref().join("header.md")) {
        let refs = comrak::parse_document_refs(&Arena::new(), &s);
        parse_dir_internal(0, input, output, &refs, &mut state)
    } else {
        parse_dir_internal(0, input, output, &RefMap::new(), &mut state)
    }
    .context("While parsing markdown files")?;

    Ok(state)
}

fn register_tracks(cfg: &str, _path: &PathBuf, _state: &mut ParseState) -> anyhow::Result<()> {
    let tracks: TracksRaw = toml::from_str(cfg)?;
    let _tracks = tracks.tracks;

    Ok(())
}

fn parse_dir_internal<P: AsRef<Path>>(
    depth: u8,
    input: P,
    output: P,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<()> {
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

    // go through all the files, copying them over and parsing them
    for entry in fs::read_dir(input)? {
        let i = entry?.path();
        let o = output.join(i.clone().strip_prefix(input).unwrap());

        // if its a dir, update links and call the fn again
        if i.is_dir() {
            if !o.exists() {
                fs::create_dir(&o)?;
            }
            // top level -> course

            // parse header.md
            if let Ok(s) = fs::read_to_string(input.join("header.md")) {
                let other_refs = comrak::parse_document_refs(&Arena::new(), &s);
                let mut refs = refs.clone();
                refs.extend(other_refs);

                // also parse header.md to add any of the things it has
                let (_, new_refs) = parse_and_refs(&i, &s, &refs, state)
                    .context(format!("While parsing {}", i.display()))?;
                parse_dir_internal(depth + 1, &i, &o, &new_refs, state)?;
            } else {
                parse_dir_internal(depth + 1, &i, &o, refs, state)?;
            }
            continue;
        }

        let ext = i.extension().unwrap_or_default();
        let name = i.file_name().unwrap_or_default();

        match ext.to_str().unwrap_or_default() {
            "md" => {
                // ignore header.md
                if name == "header.md" {
                    continue;
                }

                // otherwise, parse
                let s = fs::read_to_string(&i).context("While reading file")?;
                anyhow::ensure!(
                    depth >= 1,
                    "File: {name:?} must be in the article directory"
                );
                let config = parse_article_config(&s)
                    .context(format!("While parsing config header for {}", i.display()))?;

                let path = i.strip_prefix(config::INPUT.clone()).unwrap();
                let output = parse(path, &s, refs, state)
                    .context(format!("While parsing file {}", i.display()))?;
                state.insert_article_config(&path, config);

                fs::write(o.with_extension("html"), output)?;
            }
            "toml" if name == "config.toml" => {
                // parse the config file
                // let config = parse_config(&i)?;
            }
            _ => {
                // copy over the file
                fs::copy(i, o)?;
            }
        }
    }

    Ok(())
}

/// This function will watch the input directory and write to the output
/// directory when detecting file changes using the `notify` crate.
///
/// See [`parse_dir`] for more description on how this function behaves
pub fn parse_dir_watch(state: Arc<RwLock<ParseState>>) -> notify::Result<()> {
    let (tx, rx) = sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(config::INPUT.as_path(), notify::RecursiveMode::Recursive)?;

    info!("Watching for changes in '{}'", config::INPUT);

    while let Ok(mut event) = rx.recv() {
        use notify::EventKind::*;

        // wait 50ms to avoid duplicate events
        thread::sleep(time::Duration::from_millis(50));

        // drain the channel
        while let Ok(e) = rx.try_recv() {
            match e {
                Ok(e) if matches!(e.kind, Create(_) | Modify(_) | Remove(_)) => event = Ok(e),
                Err(e) => error!("Error watching directory: {:?}", e),
                _ => (),
            }
        }

        match event {
            Ok(event) if matches!(event.kind, Create(_) | Modify(_) | Remove(_)) => {
                info!("Change detected, reparsing...");
                match parse_dir(&config::INPUT, &config::RENDERED) {
                    Err(e) => error!("Error parsing directory: '{:?}'", e),
                    Ok(s) => *state.write() = s,
                }
            }
            Err(e) => error!("Error watching directory: {:?}", e),
            _ => (),
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
