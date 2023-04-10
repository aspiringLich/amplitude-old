use anyhow::Context;
use anyhow::ensure;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
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
    pub fn get_quiz(&self, article: &PathBuf, id: String) -> Option<&quiz::Quiz> {
        self.questions.get(&(article.clone(), id))
    }

    pub fn insert_quiz(
        &mut self,
        article: &PathBuf,
        id: &String,
        quiz: quiz::Quiz,
    ) -> Option<quiz::Quiz> {
        self.questions.insert((article.clone(), id.clone()), quiz)
    }
}

pub struct State {
    pub parse: RwLock<ParseState>,
}
