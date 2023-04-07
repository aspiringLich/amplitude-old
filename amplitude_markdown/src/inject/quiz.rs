use super::*;
use crate::parse::parse;
use amplitude_common::state::{ParseState, Quiz};
use anyhow::Context;

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
pub(super) fn inject_quiz<'a>(
    article: ArticleRef,
    args: HashMap<String, String>,
    node: &'a AstNode<'a>,
    state: &mut ParseState,
    refs: &RefMap,
) -> anyhow::Result<Vec<&'a AstNode<'a>>> {
    let id = args.get("id").context("Quiz must have an id")?;

    let val = &mut node.data.borrow_mut().value;
    match val {
        NodeValue::CodeBlock(ref code) => {
            let mut quiz: Quiz =
                toml::from_str(&code.literal).context("failed to parse toml for quiz")?;
            for question in quiz.questions.iter_mut() {
                question.question = parse(article, &question.question, refs, state)?;
                for answer in question.answers.iter_mut() {
                    answer.text = parse(article, &answer.text, refs, state)?
                        .strip_prefix("<p>")
                        .and_then(|s| s.strip_suffix("</p>\n"))
                        .context("Expected <p> tags on quiz answer")?
                        .to_string();
                    answer.response = parse(article, &answer.response, refs, state)?.strip_prefix("<p>")
                        .and_then(|s| s.strip_suffix("</p>\n"))
                        .context("Expected <p> tags on quiz response")?
                        .to_string();
                }
            }
            let key = (
                article.course.to_string(),
                article.article.to_string(),
                id.to_string(),
            );
            anyhow::ensure!(
                state.questions.insert(key, quiz).is_none(),
                "Quiz id `{id}` already exists in this file"
            )
        }
        _ => unreachable!(),
    }
    *val = NodeValue::HtmlInline(format!("<Quiz id=\"{id}\"></Quiz>"));

    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use amplitude_common::state::{Answer, Question};

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
