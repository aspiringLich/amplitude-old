use std::{collections::HashMap, sync::Mutex};

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

#[derive(Debug)]
pub struct ParseState {
    pub options: comrak::ComrakOptions,
    pub questions: HashMap<(String, String, String), Quiz>,
}

impl ParseState {
    pub fn get_question(&self, course: &str, article: &str, id: &str) -> Option<&Quiz> {
        self.questions
            .get(&(course.to_string(), article.to_string(), id.to_string()))
    }
}

pub struct State {
    pub parse: Mutex<ParseState>,
}
