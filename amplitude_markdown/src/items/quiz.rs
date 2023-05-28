
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
pub struct QuizRaw {
    pub questions: Vec<Question>,
}

#[derive(Serialize, Debug)]
pub struct Quiz {
    pub id: String,
    pub questions: Vec<Question>,
}

impl Quiz {
    pub fn from_raw(raw: QuizRaw, id: String) -> Self {
        Self {
            id,
            questions: raw.questions,
        }
    }
}

impl Item for Quiz {
    fn parse_from_dir(
        dir: &Path,
        contents: DirContents,
        _: &mut ItemContext,
    ) -> anyhow::Result<ItemType>
    where
        Self: Sized,
    {
        anyhow::ensure!(
            contents.contains("quiz.toml"),
            "Required item: `quiz.toml` not found"
        );
        anyhow::ensure!(
            contents.len() == 1,
            "Quiz directory should only contain `quiz.toml`"
        );

        let quiz_raw: QuizRaw = toml::from_str(&fs::read_to_string(dir.join("quiz.toml"))?)
            .context("While parsing quiz.toml")?;
        let id = dir.file_name().to_string();
        let quiz = Quiz::from_raw(quiz_raw, id);
        
        Ok(ItemType::Quiz(quiz))
    }
}
