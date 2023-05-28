use crate::parse::context::ItemContext;
use anyhow::Context;
use anyhow::Error;
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
    let mut errors: Vec<Error> = Vec::new();
    macro parse_item($item:ty, $name:literal) {
        match <$item>::parse_from_dir(path, &mut context)
            .with_context(|| format!("While attempting to parse as `{}`", $name))
        {
            Ok(item) => return Ok(item),
            Err(err) => errors.push(err),
        }
    }

    parse_item!(article::Article, "Article");

    anyhow::bail!(errors
        .into_iter()
        .map(|err| {
            let s = err.backtrace().to_string();
            let mut backtrace = s
                .lines()
                .skip_while(|l| !l.contains("amplitude_markdown"))
                .take_while(|l| !l.contains(std::file!()))
                .enumerate()
                .filter_map(|(i, s)| {
                    (i % 2 == 1)
                        .then(|| format!("    {}: {}", i / 2, s.trim().strip_prefix("at ").unwrap()))
                })
                .collect::<Vec<_>>()
                .join("\n");

            if backtrace.is_empty() {
                backtrace = "backtrace disabled".to_string()
            }

            let chain = err
                .chain()
                .skip(1)
                .enumerate()
                .map(|(i, e)| format!("    {i}: {e}"))
                .collect::<Vec<_>>()
                .join("\n");

            format!("{err}\n\nCaused By:\n{chain}\nBacktrace:\n{backtrace}",)
        })
        .collect::<Vec<_>>()
        .join("\n"));
}
