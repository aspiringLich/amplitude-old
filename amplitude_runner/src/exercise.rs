use crate::runner::{run, RunOutput};
use crate::{lang::Language, var_type::VariableType};
use amplitude_common::config::{Config, LanguageConfig};
use amplitude_common::path;

use anyhow::Context;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub ty: VariableType,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct DynStruct {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionConfig {
    inputs: Vec<VariableType>,
    output: VariableType,
    #[serde(skip)]
    pub seed: i64,
    #[serde(default = "hidden_cases_default")]
    pub hidden_cases: u32,
    #[serde(default = "visible_cases_default")]
    pub visible_cases: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExerciseConfig {
    title: String,
    #[serde(skip)]
    pub instructions: String,
    pub(crate) functions: HashMap<String, FunctionConfig>,
    #[serde(skip)]
    tests: TestCases,
    #[serde(skip)]
    runner: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(into = "ExcerciseSerialize")]
pub struct Exercise {
    config: ExerciseConfig,
    code: HashMap<Language, String>,
}

impl Exercise {
    pub fn new(config: ExerciseConfig, code: HashMap<Language, String>) -> Self {
        Self { config, code }
    }
}

impl Into<ExcerciseSerialize> for Exercise {
    fn into(self) -> ExcerciseSerialize {
        ExcerciseSerialize {
            title: self.config.title,
            instructions: self.config.instructions,
            code: self.code,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ExcerciseSerialize {
    title: String,
    instructions: String,
    code: HashMap<Language, String>,
}

const fn bool_is_false(b: &bool) -> bool {
    *b == false
}

pub fn runner_template(lang: &Language, cfg: &ExerciseConfig) -> anyhow::Result<String> {
    let template_file = fs::read_to_string(&path::LANGUAGES.join(lang.image()).join("runner.hbs"))
        .context("While trying to read template file")?;

    todo!();
    // let out = Handlebars::new()
    //     .render(&template_file, &json!({}))
    //     .context("While rendering template")?;
    // Ok(out)
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub inputs: Vec<serde_json::Value>,
    pub output: serde_json::Value,
    #[serde(default)]
    #[serde(skip_serializing_if = "bool_is_false")]
    pub hidden: bool,
}

type TestCases = HashMap<String, Vec<TestCase>>;

const fn hidden_cases_default() -> u32 {
    5
}

const fn visible_cases_default() -> u32 {
    5
}

pub fn generator_template(
    lang: &Language,
    content: &str,
    exercise_cfg: &ExerciseConfig,
) -> anyhow::Result<String> {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file(
        "generator",
        &path::LANGUAGES.join(lang.image()).join("generator.hbs"),
    )?;
    handlebars.register_escape_fn(handlebars::no_escape);
    let out = handlebars
        .render(
            "generator",
            &json!({
                "content": content,
                "functions": &exercise_cfg.functions,
            }),
        )
        .context("While rendering template")?;
    Ok(out)
}

pub fn generate(
    lang: &Language,
    cfg: &Config,
    content: &str,
    exercise_cfg: &mut ExerciseConfig,
) -> anyhow::Result<()> {
    let RunOutput { stdout, stderr, .. } = run(
        cfg.docker.language_config.get(lang.image()).unwrap(),
        &cfg.docker,
        &generator_template(lang, content, exercise_cfg).expect("While generating template"),
        HashMap::new(),
        "",
    )
    .context("While running generator")?;

    let mut test_cases: TestCases = serde_json::from_str(&stdout).with_context(|| {
        format!("While parsing test cases\nstdout:\n{stdout}\n\nstderr:\n{stderr}")
    })?;

    for ((func, cases), (_, cfg)) in test_cases.iter_mut().zip(&exercise_cfg.functions) {
        anyhow::ensure!(
            cases.len() == (cfg.hidden_cases + cfg.visible_cases) as usize,
            "Test cases for function `{}` < cfg.hidden_cases + cfg.visible_cases",
            func
        );
        for i in cfg.hidden_cases as usize..cases.len() {
            cases[i].hidden = true
        }
    }

    exercise_cfg.tests = test_cases;

    Ok(())
}

#[cfg(test)]
mod test {
    use amplitude_common::config_and_set_path;

    use super::*;

    #[test]
    fn test_generate() {
        let config = config_and_set_path().unwrap();

        let mut exercise_cfg = ExerciseConfig {
            title: "test".to_string(),
            instructions: "test".to_string(),
            functions: HashMap::from_iter([(
                "test".to_string(),
                FunctionConfig {
                    inputs: vec![VariableType::Int],
                    output: VariableType::Int,
                    seed: 0,
                    hidden_cases: 2,
                    visible_cases: 2,
                },
            )]),
            tests: TestCases::default(),
            runner: "test".to_string(),
        };

        generate(
            &Language::Python,
            &config,
            "def gen_test(ctx):\n    ctx.inputs([1])\n    ctx.output(1)\n",
            &mut exercise_cfg,
        )
        .expect("Errors in generation");

        let tests = &exercise_cfg.tests["test"];
        assert!(!tests[0].hidden);
        assert!(!tests[1].hidden);
        assert!(tests[2].hidden);
        assert!(tests[3].hidden);

        for test in tests {
            assert!(test.inputs.len() == 1);
            assert!(test.inputs[0] == 1);
            assert!(test.output == 1);
        }
    }
}
