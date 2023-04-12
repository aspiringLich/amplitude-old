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

impl ParseState {
    pub fn get_quiz(&self, article: &Path, id: String) -> Option<&quiz::Quiz> {
        self.quizzes.get(&(article.to_path_buf(), id))
    }

    pub fn insert_quiz(
        &mut self,
        article: &Path,
        id: &str,
        quiz: quiz::Quiz,
    ) -> Option<quiz::Quiz> {
        self.quizzes
            .insert((article.to_path_buf(), id.to_owned()), quiz)
    }
}
