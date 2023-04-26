use anyhow::Context;
use comrak::RefMap;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::parse::parse;
use crate::state::index::Children;

pub mod article;
pub mod index;
pub mod quiz;

#[derive(Debug, Default)]
pub struct ParseState {
    pub options: comrak::ComrakOptions,
    quizzes: HashMap<(PathBuf, String), quiz::Quiz>,
    article_config: HashMap<PathBuf, article::ArticleConfig>,
    course_config: HashMap<String, index::CourseConfig>,
    pub children: index::Children,
}

impl ParseState {
    pub fn finalize(&mut self) -> anyhow::Result<()> {
        for child in &self.children {
            let course = &child.id;
            for child in &child.children {
                let track = &child.id;
                let mut config = self
                    .course_config
                    .get_mut(course)
                    .context("course config not found")?
                    .children
                    .get_mut(track)
                    .context("track config not found")?;
                
                // order children by ord
                fn order(children: &mut Children) {
                    children.sort_by_key(|a| a.ord)
                }

                config.children = child.children.clone();
            }
        }

        Ok(())
    }

    pub fn parse(&mut self, input: &str, refs: Option<RefMap>) -> anyhow::Result<String> {
        parse(
            PathBuf::new(),
            input,
            &refs.unwrap_or_else(|| RefMap::new()),
            self,
        )
    }
}
