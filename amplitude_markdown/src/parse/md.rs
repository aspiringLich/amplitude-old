use crate::{
    config::article::ArticleConfig,
    inject::{self},
};
use amplitude_common::config::ParseConfig;
use anyhow::Context;
use comrak::{parse_document_refs, Arena, ComrakOptions, RefMap};
use git2::build::RepoBuilder;
use std::{fs, path::Path};
use tracing::warn;

use super::link_concat::link_concat_callback;

/// Parse the input `md` and return the output `html`.
pub(crate) fn parse_md(
    config: &ArticleConfig,
    input: &str,
    refs: &RefMap,
    state: &mut ParseState,
) -> anyhow::Result<(String, RefMap)> {
    // get the refs
    let mut this_refs = parse_document_refs(&Arena::new(), input);
    if !this_refs.map.is_empty() {
        this_refs.extend(refs.clone());
    } else {
        this_refs = refs.clone();
    }

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
