use pulldown_cmark::{CowStr, Event};
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
pub(super) fn inject_quiz(input: &[&str], events: &mut Vec<Event>) -> anyhow::Result<()> {
    let str = input[0];
    let quiz: Quiz = toml::from_str(str)?;
    let question = quiz.question;
    let mut answers = String::new();

    for answer in quiz.answers {
        let text = answer.text;
        answers += &format!(
            r#"
<div>
<input type="radio" value="{text}" name="quiz-answer">
<label for="{text}">{text}</label>
</div>
"#
        );
    }

    events.push(Event::Html(CowStr::Boxed(
        format!(
            r#"
<form>
<p> {question} </p>
{answers}
</form>
"#
        )
        .into_boxed_str(),
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

        assert_eq!(
            toml::from_str::<Quiz>(toml).unwrap(),
            Quiz {
                question: "e".to_string(),
                answers: vec![Answer {
                    text: "answer".to_string(),
                    response: "woo".to_string(),
                    correct: false,
                }]
            }
        )
    }
}
