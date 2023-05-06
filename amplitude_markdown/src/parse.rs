use std::{collections::VecDeque, default::default, fs, path::Path};

use amplitude_common::{config::ParseConfig, path};
use anyhow::Context;

use git2::build::RepoBuilder;
use tracing::warn;

use crate::{
    inject::{self},
    link_concat::link_concat_callback,
    state::ParseState,
    util::get_path_components,
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

pub fn parse_init(config: &ParseConfig) {}

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

    let (_, refs) = parse_and_refs(
        input.join("header.md"),
        &fs::read_to_string(input.join("header.md"))?,
        &RefMap::new(),
        &mut state,
    )
    .context("parsing top-level header file")?;

    // recursively parse input directory, skipping
    // top-level index files
    let mut dirs = VecDeque::new();
    for item in fs::read_dir(input)? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            dirs.push_back(path);
        }
    }

    Ok(())
}

fn internal_parse_dir(config: &ParseConfig) {
    
} 