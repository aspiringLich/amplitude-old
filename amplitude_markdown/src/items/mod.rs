use crate::items::utils::ErrorList;
use crate::parse::context::ItemContext;
use anyhow::Context;

use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod article;
pub mod quiz;
pub mod utils;

#[derive(Debug)]
pub enum ItemType {
    Article(article::Article),
    Quiz(quiz::Quiz),
}

pub trait Item {
    fn parse_from_dir(dir: &Path, context: &mut ItemContext) -> anyhow::Result<ItemType>
    where
        Self: Sized;
}

pub fn parse_item(path: &Path, mut context: ItemContext) -> anyhow::Result<ItemType> {
    let mut errors = ErrorList::new("Could not parse as valid item", file!());
    macro parse_item($item:ty, $name:literal) {
        match <$item>::parse_from_dir(path, &mut context)
            .with_context(|| format!("While attempting to parse as `{}`", $name))
        {
            Ok(item) => return Ok(item),
            Err(err) => errors.push(err),
        }
    }

    parse_item!(article::Article, "Article");

    anyhow::bail!(errors)
}
