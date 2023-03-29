use anyhow::Context;
use serde::Deserialize;

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
struct Quiz {
    question: String,
    answers: Vec<Answer>,
}

/// Turns a code block into a quiz
///
/// ```compile_fail
///  @quiz
///  ```toml
///  question = "What is the answer to life, the universe, and everything?"
///  
///  [[answers]]
///  text = "42"
///  correct = true
///  response = "42 is, in fact, the answer to life, the universe and everything"
///  
///  [[answers]]
///  text = "30"
///  response = "hint: 30 is too low"
///  
///  [[answers]]
///  text = "41"
///  response = "41 is close, but not quite"
///  
///  [[answers]]
///  text = "0"
///  reponse = "OK nihilist"
///  ```
/// ```
pub(super) fn inject_quiz(
    input: Vec<Event>,
    id: &str,
    events: &mut Vec<Event>,
    state: &mut ParseState,
) -> anyhow::Result<()> {
    if input.len() != 3 {
        anyhow::bail!("internal error: input len should be 3");
    }
    if id.trim().is_empty() {
        anyhow::bail!("empty id!")
    }
    let Event::Text(str) = &input[1] else { unreachable!() };
    // dbg!(str);

    let quiz: Quiz = toml::from_str(str).context("While parsing quiz TOML")?;
    let mut answers = String::new();

    for (i, answer) in quiz.answers.iter().enumerate() {
        let text = parse(&answer.text, state.links).context("While parsing answers for quiz")?;
        let text = text
            .strip_prefix("<p>")
            .and_then(|s| s.strip_suffix("</p>\n"))
            .unwrap_or(&text);
        answers += &format!(
            r#"<div>
<input type="radio" value="{i}">
<label for="{i}">{text}</label>
</div>
"#
        );
    }

    let id = id.trim();
    let question = parse(&quiz.question, state.links).context("While parsing quiz question")?;
    let question = question.strip_prefix("<p>")
            .and_then(|s| s.strip_suffix("</p>\n"))
            .unwrap_or(&question);
    events.push(Event::Html(CowStr::Boxed(
        format!("<form id=\"{id}\">\n{question}{answers}</form>").into_boxed_str(),
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
