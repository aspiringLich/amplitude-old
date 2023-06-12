use crate::{lang::Language, var_type::VariableType};
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

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionConfig {
    inputs: Vec<VariableType>,
    output: VariableType,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExerciseConfig {
    title: String,
    #[serde(default)]
    test: TestCaseConfig,
    #[serde(skip)]
    pub instructions: String,
    pub(crate) functions: HashMap<String, FunctionConfig>,
    tests: TestCases,
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub inputs: Vec<serde_json::Value>,
    pub output: serde_json::Value,
    #[serde(default)]
    #[serde(skip_serializing_if = "bool_is_false")]
    pub hidden: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TestCases {
    #[serde(flatten)]
    pub tests: HashMap<String, Vec<TestCase>>,
}

const fn hidden_cases_default() -> u32 {
    5
}

const fn visible_cases_default() -> u32 {
    5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestCaseConfig {
    #[serde(default = "hidden_cases_default")]
    pub hidden_cases: u32,
    #[serde(default = "visible_cases_default")]
    pub visible_cases: u32,
    #[serde(skip)]
    pub seed: i64,
    #[serde(default)]
    pub tests: TestCases,
}

impl Default for TestCaseConfig {
    fn default() -> Self {
        Self {
            hidden_cases: hidden_cases_default(),
            visible_cases: visible_cases_default(),
            seed: 0,
            tests: TestCases::default(),
        }
    }
}

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub fn runner_template(lang: &Language, cfg: &TestCaseConfig) -> anyhow::Result<String> {
    let template_file = fs::read_to_string(&path::LANGUAGES.join(lang.image()).join("runner.hbs"))
        .context("While trying to read template file")?;

    let out = Handlebars::new()
        .render(&template_file, &cfg)
        .context("While rendering template")?;
    Ok(out)
}

pub fn generator_template(
    lang: &Language,
    content: &str,
    exercise_cfg: &ExerciseConfig,
) -> anyhow::Result<String> {
    let template_file =
        fs::read_to_string(&path::LANGUAGES.join(lang.image()).join("generator.hbs"))
            .context("While trying to read template file")?;

    let out = Handlebars::new()
        .render(
            &template_file,
            &json!({
                "seed": exercise_cfg.test.seed,
                "hidden_cases": exercise_cfg.test.hidden_cases,
                "visible_cases": exercise_cfg.test.visible_cases,
                "content": content,
                "functions": exercise_cfg.functions.keys().collect::<Vec<_>>(),
            }),
        )
        .context("While rendering template")?;
    Ok(out)
}

pub fn generate() {}
