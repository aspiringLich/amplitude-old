pub mod course;
mod inject;
mod link_concat;
pub mod md;

use amplitude_common::config::{Config, ParseConfig};
use anyhow::{ensure, Context};
use comrak::{ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions, ListStyleType, RefMap};
use git2::build::RepoBuilder;
use std::{default::default, fs, path::Path};
use tracing::info;

use self::md::parse_md;

/// Reparses the things and does the things
pub fn parse(config: &Config) -> anyhow::Result<ParseState> {
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
        if path.is_dir() {
            
        }
    }

    dbg!(&state);

    Ok(state)
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
