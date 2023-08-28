
use crate::parse::context::DataContext;
use crate::path::{DirectoryContent, FileType, FromDirectory, FromFile};
use amplitude_common::config::Config;
use anyhow::Context;

use serde::{Deserialize, Serialize};

use std::fs::{self, File};


pub mod article;
pub mod exercise;
pub mod quiz;
pub mod utils;

// #[derive(Debug, Serialize, Clone)]
// #[serde(tag = "type")]
// #[serde(rename_all = "snake_case")]
// pub enum ItemType {
//     Article(article::Article),
//     Quiz(quiz::Quiz),
//     Exercise(exercise::Exercise),
// }

// impl ItemType {
//     pub fn serialize_for_route<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
//         let mut copy = self.clone();
//         match &mut copy {
//             ItemType::Exercise(e) => {
//                 exercise::transform(e);
//                 copy.serialize(s)
//             }
//             _ => copy.serialize(s),
//         }
//     }
// }

// impl fmt::Display for ItemType {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Self::Article(_) => "Article",
//                 Self::Quiz(_) => "Quiz",
//                 Self::Exercise(_) => "Exercise",
//             }
//         )
//     }
// }

// pub fn parse_item(
//     path: &Path,
//     mut context: &mut DataContext,
//     track_id: &str,
//     cfg: &Config,
// ) -> anyhow::Result<()> {
//     let mut errors = ErrorList::new("Could not parse as valid item", file!());
//     macro parse_item($item:ty, $name:literal, $item_type:ident) {
//         match <$item>::from_file(
//             path.file_name().unwrap().to_str().unwrap(),
//             &mut File::open(path).context("While opening file")?,
//             &mut context,
//             cfg,
//         )
//         .with_context(|| format!("While attempting to parse as `{}`", $name))
//         {
//             Ok(item) => {
//                 // debug!("{:#?}", &item);
//                 context
//                     .add_item(ItemType::$item_type(item), track_id)
//                     .context("While adding item to context")?;
//                 return Ok(());
//             }
//             Err(err) => errors.push(err),
//         }
//     }

//     parse_item!(article::Article, "Article", Article);
//     parse_item!(quiz::Quiz, "Quiz", Quiz);

//     anyhow::bail!(errors)
// }
