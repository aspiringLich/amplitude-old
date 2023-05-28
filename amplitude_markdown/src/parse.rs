pub mod context;
pub mod course;
mod inject;
mod link_concat;
pub mod output;

use crate::{
    parse::{course::parse_course},
    OsStrToString,
};
use amplitude_common::config::{Config, ParseConfig};
use anyhow::{Context};
use comrak::{
    parse_document_refs, Arena, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions,
    ListStyleType, RefMap,
};
use git2::build::RepoBuilder;
use link_concat::link_concat_callback;
use std::{
    collections::{HashMap},
    default::default,
    fs,
    path::Path,
};
use tracing::{info, warn};

use self::{
    context::{ItemContext, MarkdownContext},
    output::ParseData,
};

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

    // delete everything in the folder
    for item in fs::read_dir(&config.parse.output_path)? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    
    info!("Parsing articles...");

    let mut courses = HashMap::new();
    for item in fs::read_dir(&config.parse.clone_path)? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            let name = path.file_name().to_string();
            if name.starts_with('.') {
                continue;
            }

            let context = parse_course(path, config)
                .with_context(|| format!("While parsing course `{name}`"))?;
            courses.insert(name, context);
        }
    }
    let data = ParseData::from_courses(courses, config).context("While generating `ParseData`")?;

    dbg!(&data);

    Ok(data)
}

/// Parse the input `md` and return the output `html`.
/// Has full access to `ItemContext`
pub(crate) fn parse_md(input: &str, ctx: &mut ItemContext) -> anyhow::Result<String> {
    // get the refs
    let mut this_refs = parse_document_refs(&Arena::new(), input);
    this_refs.extend(ctx.markdown_context().refs.clone());

    let options = unsafe { &*(ctx.markdown_options() as *const ComrakOptions) };

    let arena = Arena::new();
    let out = comrak::parse_document_with_broken_link_callback(
        &arena,
        input,
        ctx.markdown_options(),
        Some(&mut |link| {
            let out = link_concat_callback(link, &this_refs);
            if out.is_none() {
                warn!("Broken link `{link}` in {:?}", ctx.id());
            }
            out
        }),
    );
    // do things
    inject::inject(out, ctx)?;

    let mut cm = vec![];
    comrak::format_html(out, options, &mut cm).context("while parsing AST to html")?;

    Ok(String::from_utf8(cm).unwrap())
}
