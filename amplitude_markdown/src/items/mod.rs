use crate::items::utils::ErrorList;
use crate::parse::context::ItemContext;
use anyhow::Context;

use crate::OsStrToString;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub mod article;
pub mod exercise;
pub mod project;
pub mod quiz;
pub mod utils;

use utils::*;

#[derive(Debug)]
pub enum ItemType {
    Article(article::Article),
    Quiz(quiz::Quiz),
    Exercise(exercise::Exercise),
    Project(project::Project),
}

pub trait Item {
    fn parse_from_dir(
        dir: &Path,
        contents: DirContents,
        context: &mut ItemContext,
    ) -> anyhow::Result<ItemType>;
}

pub fn parse_item(path: &Path, mut context: ItemContext) -> anyhow::Result<ItemType> {
    let mut errors = ErrorList::new("Could not parse as valid item", file!());
    macro parse_item($item:ty, $name:literal) {
        match <$item>::parse_from_dir(
            path,
            get_dir_contents(path).context("While reading dir")?,
            &mut context,
        )
        .with_context(|| format!("While attempting to parse as `{}`", $name))
        {
            Ok(item) => return Ok(item),
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
    ($cond:expr, $file:literal) => {
        if !$cond {
            anyhow::bail!(
                "Required file(s): `{}` not found",
                $file
            );
        }
    },
    ($cond:expr, $file:literal, $ctx:literal) => {
        if !$cond {
            anyhow::bail!(
                "Required file(s): `{}` ({}) not found",
                $file,
                $ctx
            );
        }
    }
}
