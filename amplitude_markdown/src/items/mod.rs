use crate::parse::context::ItemContext;
use crate::parse::course::CourseContext;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::Path;
use utils::DirItem;

pub mod article;
pub mod quiz;
pub mod utils;

pub trait Item {
    fn parse_from_dir(dir: &Path, context: &mut ItemContext) -> anyhow::Result<Self>
    where
        Self: Sized;
}
