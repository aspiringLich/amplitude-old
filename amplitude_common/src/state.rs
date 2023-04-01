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
    pub questions: HashMap<(String, String, String), Quiz>,
}

pub struct State {
    pub parse: Mutex<ParseState>,
}
