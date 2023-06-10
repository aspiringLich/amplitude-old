use crate::items::utils::ErrorList;
use crate::parse::context::DataContext;
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

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Article(article::Article),
    Quiz(quiz::Quiz),
    Exercise(exercise::Exercise),
    Project(project::Project),
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
    ) -> anyhow::Result<ItemType>;
}

pub fn parse_item(
    path: &Path,
    mut context: &mut DataContext,
    track_id: &str,
) -> anyhow::Result<()> {
    let mut errors = ErrorList::new("Could not parse as valid item", file!());
    macro parse_item($item:ty, $name:literal) {
        match <$item>::parse_from_dir(
            path,
            DirContents::new(path).context("While getting dir contents")?,
            &mut context,
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
