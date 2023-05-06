use anyhow::Context;
use comrak::RefMap;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::parse::parse_md;

pub mod article;
pub mod quiz;

/// Stores the state of the parsing process.
#[derive(Debug, Default)]
pub struct ParseState {
    pub options: comrak::ComrakOptions,
    quizzes: HashMap<(String, String), quiz::Quiz>,
    pub article_ids: HashMap<String, PathBuf>,
}

impl ParseState {
    pub fn get_quiz(&self, article_id: String, quiz_id: String) -> Option<&quiz::Quiz> {
        self.quizzes.get(&(article_id, quiz_id))
    }

    pub fn insert_quiz(
        &mut self,
        article: &str,
        id: &str,
        quiz: quiz::Quiz,
    ) -> Option<quiz::Quiz> {
        self.quizzes
            .insert((article.to_string(), id.to_string()), quiz)
    }
}
