use std::io::Read;

use crate::parse::context::ParseMarkdown;

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Answer {
    pub text: String,
    #[serde(default)]
    pub response: String,
    #[serde(default)]
    pub correct: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Serialize, Debug, Clone)]
pub struct Quiz {
    pub id: String,
    pub questions: Vec<Question>,
}

impl ParseMarkdown for Quiz {
    fn parse_md(&mut self, ctx: &mut DataContext) -> anyhow::Result<()> {
        for question in &mut self.questions {
            ctx.parse_md(&mut question.question)?;
            for answer in &mut question.answers {
                ctx.parse_md(&mut answer.text)?;
                ctx.parse_md(&mut answer.response)?;
            }
        }
        Ok(())
    }
}

impl Quiz {
    fn from_raw(raw: QuizRaw, id: String, ctx: &mut DataContext) -> anyhow::Result<Self> {
        ctx.scope(&id.clone(), |ctx| {
            let mut out = Self {
                id,
                questions: raw.questions,
            };
            ctx.parse_md(&mut out).context("While parsing markdown")?;
            Ok(out)
        })
    }

    pub fn from_str(s: &str, id: String, ctx: &mut DataContext) -> anyhow::Result<Self> {
        let raw: QuizRaw = toml::from_str(s).context("While parsing quiz toml")?;
        Self::from_raw(raw, id, ctx)
    }
}

impl FromFile for Quiz {
    fn from_file(
        filename: &str,
        file: &mut File,
        context: &mut DataContext,
        _: &Config,
    ) -> anyhow::Result<Self> {
        let id = filename[3..]
            .strip_suffix(".toml")
            .unwrap_or("")
            .to_string();
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let quiz = Quiz::from_str(&s, id, context)?;

        Ok(quiz)
    }
}
