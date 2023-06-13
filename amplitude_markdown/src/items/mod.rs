use crate::items::utils::ErrorList;
use crate::parse::context::DataContext;
use amplitude_common::config::Config;
use anyhow::Context;

use crate::OsStrToString;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::Path;

pub mod article;
pub mod exercise;
pub mod project;
pub mod quiz;
pub mod utils;

use utils::*;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Article(article::Article),
    Quiz(quiz::Quiz),
    Exercise(exercise::Exercise),
    Project(project::Project),
}

impl ItemType {
    pub fn serialize_for_route<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut copy = self.clone();
        match &mut copy {
            ItemType::Article(a) => a.transform(),
            ItemType::Quiz(q) => q.transform(),
            ItemType::Exercise(e) => e.transform(),
            ItemType::Project(p) => p.transform(),
        }
        copy.serialize(s)
    }
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Article(_) => "Article",
                Self::Quiz(_) => "Quiz",
                Self::Exercise(_) => "Exercise",
                Self::Project(_) => "Project",
            }
        )
    }
}

pub trait Item {
    /// Parse an item given a directory
    fn parse_from_dir(
        dir: &Path,
        contents: DirContents,
        context: &mut DataContext,
        cfg: &Config,
    ) -> anyhow::Result<ItemType>;

    /// Transform an instance of self into one ready to jsonify and send
    /// to the client
    fn transform(&mut self) {}
}

pub fn parse_item(
    path: &Path,
    mut context: &mut DataContext,
    track_id: &str,
    cfg: &Config,
) -> anyhow::Result<()> {
    let mut errors = ErrorList::new("Could not parse as valid item", file!());
    macro parse_item($item:ty, $name:literal) {
        match <$item>::parse_from_dir(
            path,
            DirContents::new(path).context("While getting dir contents")?,
            &mut context,
            cfg,
        )
        .with_context(|| format!("While attempting to parse as `{}`", $name))
        {
            Ok(item) => {
                // debug!("{:#?}", &item);
                context
                    .add_item(item, track_id)
                    .context("While adding item to context")?;
                return Ok(());
            }
            Err(err) => errors.push(err),
        }
    }

    parse_item!(article::Article, "Article");
    parse_item!(quiz::Quiz, "Quiz");
    parse_item!(exercise::Exercise, "Exercise");
    parse_item!(project::Project, "Project");

    anyhow::bail!(errors)
}

macro ensure{
    ($cond:expr, $file:expr) => {
        if !$cond {
            anyhow::bail!(
                "Required file(s): `{}` not found",
                $file
            );
        }
    },
    ($cond:expr, $file:expr, $ctx:expr) => {
        if !$cond {
            anyhow::bail!(
                "Required file(s): `{}` ({}) not found",
                $file,
                $ctx
            );
        }
    }
}
