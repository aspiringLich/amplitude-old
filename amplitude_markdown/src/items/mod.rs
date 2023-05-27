use crate::parse::course::CourseContext;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub mod article;
pub mod quiz;
pub mod track;

pub trait Item {
    fn parse_from_dir(dir: &Path, context: &CourseContext) -> anyhow::Result<Self>
    where
        Self: Sized;
}
