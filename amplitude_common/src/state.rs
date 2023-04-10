use anyhow::Context;
use anyhow::ensure;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Mutex, RwLock},
    fs,
};

pub mod config;
pub mod quiz;

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ParseState {
    pub options: comrak::ComrakOptions,
    pub questions: HashMap<(PathBuf, String), quiz::Quiz>,
}

impl ParseState {
    pub fn get_quiz(&self, article: &Path, id: String) -> Option<&quiz::Quiz> {
        self.questions.get(&(article.to_path_buf(), id))
    }

    pub fn insert_quiz(
        &mut self,
        article: &Path,
        id: &str,
        quiz: quiz::Quiz,
    ) -> Option<quiz::Quiz> {
        self.questions
            .insert((article.to_path_buf(), id.to_owned()), quiz)
    }
}

pub struct State {
    pub parse: RwLock<ParseState>,
}
