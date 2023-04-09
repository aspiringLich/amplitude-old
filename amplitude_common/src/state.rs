use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::Mutex,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Answer {
    pub text: String,
    #[serde(default)]
    pub response: String,
    #[serde(default)]
    pub correct: bool,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Question {
    pub question: String,
    pub answers: Vec<Answer>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Quiz {
    pub questions: Vec<Question>,
}

const DEPTH_NAMES: [&str; 3] = ["course", "track", "article"];

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
    pub questions: HashMap<(Vec<String>, String), Quiz>,
}

impl ParseState {
    pub fn get_question(&self, article: ArticleRef, id: String) -> Option<&Quiz> {
        self.questions.get(&(article.levels, id))
    }

    pub fn insert_question<'a>(&'a mut self, article: &ArticleRef, id: &String, quiz: Quiz) -> Option<Quiz> {
        self.questions
            .insert((article.levels.clone(), id.clone()), quiz)
    }
}

pub struct State {
    pub parse: Mutex<ParseState>,
}
