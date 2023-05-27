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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawQuizConfig {
    pub questions: Vec<Question>,
}

#[derive(Serialize, Debug)]
pub struct Quiz {
    pub id: String,
    pub questions: Vec<Question>,
}

impl Quiz {
    pub fn from_str(str: &str, id: String) -> anyhow::Result<Self> {
        let raw: RawQuizConfig = toml::from_str(str)?;
        Ok(Self {
            id,
            questions: raw.questions,
        })
    }
}