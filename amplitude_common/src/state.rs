use anyhow::Context;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::Mutex,
};

pub mod config;
pub mod quiz;

use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct ArticleRef {
    pub levels: Vec<String>,
}

impl Deref for ArticleRef {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.levels
    }
}

impl DerefMut for ArticleRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.levels
    }
}

#[derive(Debug)]
pub struct ParseState {
    pub options: comrak::ComrakOptions,
    pub questions: HashMap<(Vec<String>, String), quiz::Quiz>,
}

impl ParseState {
    pub fn get_quiz(&self, article: ArticleRef, id: String) -> Option<&quiz::Quiz> {
        self.questions.get(&(article.levels, id))
    }

    pub fn insert_quiz<'a>(
        &'a mut self,
        article: &ArticleRef,
        id: &String,
        quiz: quiz::Quiz,
    ) -> Option<quiz::Quiz> {
        self.questions
            .insert((article.levels.clone(), id.clone()), quiz)
    }
}

pub struct State {
    pub parse: Mutex<ParseState>,
}
