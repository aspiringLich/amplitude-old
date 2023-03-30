use super::*;
use crate::parse::parse;
use anyhow::Context;
use serde::Deserialize;

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

#[derive(Debug, PartialEq, Deserialize)]
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
    state: &mut ParseState,
    refs: &RefMap,
) -> anyhow::Result<()> {
    if id.trim().is_empty() {
        anyhow::bail!("empty id! You should have something like `@quiz;id`");
    }

    let val = &mut node.data.borrow_mut().value;
    match val {
        NodeValue::CodeBlock(ref code) => {
            let mut quiz: Quiz = toml::from_str(&code.literal).context("failed to parse quiz")?;
            for question in quiz.questions.iter_mut() {
                question.question = parse(article, &question.question, refs, state)?;
                for answer in question.answers.iter_mut() {
                    answer.text = parse(article, &answer.text, refs, state)?;
                    answer.response = parse(article, &answer.response, refs, state)?;
                }
            }
            state
                .insert_question(article, id, quiz)
                .is_none()
                .then(|| ())
                .context(format!("Quiz id `{id}` already exists in this file"))?;
        }
        _ => unreachable!(),
    }
    *val = NodeValue::HtmlInline(format!("<Quiz id=\"{id}\"></Quiz>"));

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
