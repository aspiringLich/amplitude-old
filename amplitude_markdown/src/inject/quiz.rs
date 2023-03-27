use anyhow::Context;
use pulldown_cmark::{html, CowStr, Event};
use serde::Deserialize;

use crate::{link_concat::LinkDefs, parse::parse};

use super::ParseState;

#[derive(Deserialize, Debug, PartialEq)]
struct Answer {
    text: String,
    #[serde(default)]
    response: String,
    #[serde(default)]
    correct: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Quiz {
    question: String,
    answers: Vec<Answer>,
}

/// Turns a code block into a quiz
///     
///     @quiz
///     ```toml
///     question = "What is the answer to life, the universe, and everything?"
///     
///     [[answers]]
///     text = "42"
///     correct = true
///     response = "42 is, in fact, the answer to life, the universe and everything"
///     
///     [[answers]]
///     text = "30"
///     response = "hint: 30 is too low"
///     
///     [[answers]]
///     text = "41"
///     response = "41 is close, but not quite"
///     
///     [[answers]]
///     text = "0"
///     reponse = "OK nihilist"
///     ```
pub(super) fn inject_quiz(
    input: Vec<Event>,
    _: &str,
    events: &mut Vec<Event>,
    state: &mut ParseState,
) -> anyhow::Result<()> {
    assert!(input.len() == 3);
    let Event::Text(str) = &input[1] else { unreachable!() };
    // dbg!(str);

    let quiz: Quiz = toml::from_str(str).context("While parsing quiz TOML")?;
    let mut answers = String::new();

    for (i, answer) in quiz.answers.iter().enumerate() {
        let text = parse(&answer.text, state.links).context("While parsing answers for quiz")?;
        answers += &format!(
            r#"
            <div>
            <input type="radio" value="{i}" name="quiz-answer">
            <label for="{i}">{text}</label>
            </div>
            "#
        );
    }

    let question = parse(&quiz.question, state.links).context("While parsing quiz question")?;
    events.push(Event::Html(CowStr::Boxed(
        format!("<form>{question}{answers}</form>").into_boxed_str(),
    )));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::inject::quiz::{Answer, Quiz};

    #[test]
    fn test_serde() {
        let toml = r#"
question = "e"

[[answers]]
text = "answer"
response = "woo"
"#;
        let quiz = Quiz {
            question: "e".to_string(),
            answers: vec![Answer {
                text: "answer".to_string(),
                response: "woo".to_string(),
                correct: false,
            }],
        };

        assert_eq!(toml::from_str::<Quiz>(toml).unwrap(), quiz)
    }
}
