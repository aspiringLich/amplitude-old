use anyhow::Context;
use serde::Deserialize;
use super::*;
use crate::parse::parse;

#[derive(Deserialize, Debug, PartialEq)]
struct Answer {
    text: String,
    #[serde(default)]
    response: String,
    #[serde(default)]
    correct: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Question {
    question: String,
    answers: Vec<Answer>,
}

#[derive(Debug, PartialEq)]
pub struct Quiz {
    pub questions: Vec<Question>,
}

/// Turns a code block into a quiz
///
/// ````compile_fail
/// @quiz
/// ```toml
/// [[questions]]
/// question = "What is the meaning to life, the universe, and everything?"
/// answers = [
///     { text = "42", response = "Yes!", correct = true },
///     { text = "24", response = "You got it backwards" },
///     { text = "41", response = "Close, but not quite" },
///     { text = "43", response = "Nope" },
/// ]
/// ``` 
/// ````
pub(super) fn inject_quiz(
    article: ArticleRef,
    id: &str,
    node: &AstNode<'_>,
    state: &mut ParseState<'_>,
) -> anyhow::Result<()> {
    if id.trim().is_empty() {
        anyhow::bail!("empty id! You should have something like `@quiz;id`");
    }
    

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::inject::quiz::{Answer, Question};

    #[test]
    fn test_serde() {
        let toml = r#"
question = "e"

[[answers]]
text = "answer"
response = "woo"
"#;
        let quiz = Question {
            question: "e".to_string(),
            answers: vec![Answer {
                text: "answer".to_string(),
                response: "woo".to_string(),
                correct: false,
            }],
        };

        assert_eq!(toml::from_str::<Question>(toml).unwrap(), quiz)
    }
}
