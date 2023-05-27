pub mod context;
pub mod course;
mod inject;
mod link_concat;
pub mod output;
mod track;

use crate::items::article::ArticleConfig;
use amplitude_common::config::{Config, ParseConfig};
use anyhow::{ensure, Context};
use comrak::{
    parse_document_refs, Arena, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions,
    ListStyleType, RefMap,
};
use git2::build::RepoBuilder;
use link_concat::link_concat_callback;
use std::{collections::HashSet, default::default, fs, path::Path};
use tracing::{info, warn};

use self::{context::{ItemContext, MarkdownContext}, output::ParseData};

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

/// Reparses the things and does the things
pub fn parse(config: &Config) -> anyhow::Result<ParseData> {
    if !config.args.local {
        info!("Deleting `{}` and recloning repo... (If you dont want this behavior, run with `--local`)", config.parse.clone_path);
        clone_repo(&config.parse).context("While cloning repo")?;
    } else {
        info!(
            "Using local files in `{}` for articles",
            config.parse.clone_path
        );
    }

    info!("Parsing articles...");
    for item in fs::read_dir(&config.parse.clone_path)? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {}
    }

    dbg!(&state);

    Ok(state)
}

/// Parse the input `md` and return the output `html`.
/// Has full access to `ItemContext`
pub(crate) fn full_parse_md(
    config: &ArticleConfig,
    input: &str,
    ctx: &mut ItemContext,
) -> anyhow::Result<String> {
    let md_ctx = ctx.markdown_context();
    
    // get the refs
    let mut this_refs = parse_document_refs(&Arena::new(), input);
    this_refs.extend(md_ctx.refs.clone());

    let arena = Arena::new();
    let out = comrak::parse_document_with_broken_link_callback(
        &arena,
        input,
        md_ctx.options,
        Some(&mut |link| {
            let out = link_concat_callback(link, &this_refs);
            if out.is_none() {
                warn!("Broken link `{}` in {:?}", link, config.id);
            }
            out
        }),
    );
    // do things
    inject::inject(config, out, &this_refs, ctx)?;

    let mut cm = vec![];
    comrak::format_html(out, md_ctx.options, &mut cm).context("while parsing AST to html")?;

    Ok(String::from_utf8(cm).unwrap())
}
