use super::*;

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Answer {
    pub text: String,
    #[serde(default)]
    pub response: String,
    #[serde(default)]
    pub correct: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Question {
    pub question: String,
    pub answers: Vec<Answer>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Quiz {
    pub id: String,
    pub questions: Vec<Question>,
}
