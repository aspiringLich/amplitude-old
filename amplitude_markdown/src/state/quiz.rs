use super::*;

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
