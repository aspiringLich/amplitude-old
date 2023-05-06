use super::*;
use crate::{
    parse::parse_md,
    state::{article::ArticleConfig, quiz::Quiz},
};
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
    config: &ArticleConfig,
    args: &HashMap<String, String>,
    node: &AstNode<'a>,
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
                question.question = parse_md(config, &question.question, refs, state)?.0;
                for answer in question.answers.iter_mut() {
                    let (s, _) = parse_md(config, &answer.text, refs, state)?;
                    answer.text = s
                        .strip_prefix("<p>")
                        .and_then(|s| s.strip_suffix("</p>\n"))
                        .unwrap_or(&s)
                        .to_string();

                    let (s, _) = parse_md(config, &answer.response, refs, state)?;
                    answer.response = s
                        .strip_prefix("<p>")
                        .and_then(|s| s.strip_suffix("</p>\n"))
                        .unwrap_or(&s)
                        .to_string();
                }
            }
            anyhow::ensure!(
                state.insert_quiz(&config.id, id, quiz).is_none(),
                "Quiz id `{id}` already exists in this file"
            )
        }
        _ => unreachable!(),
    }
    *val = NodeValue::HtmlInline(format!("<Quiz id=\"{id}\"></Quiz>\n"));

    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use crate::state::quiz::{Answer, Question};

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
