use crate::runner::{run, RunOutput};
use crate::{lang::Language, var_type::VariableType};
use amplitude_common::config::Config;
use amplitude_common::path;

use anyhow::Context;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;

use std::collections::HashMap;

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
    pub seed: u64,
    #[serde(default = "hidden_cases_default")]
    pub hidden_cases: u32,
    #[serde(default = "visible_cases_default")]
    pub visible_cases: u32,
    #[serde(skip_deserializing)]
    pub tests: Vec<TestCase>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExerciseConfig {
    title: String,
    #[serde(skip)]
    pub instructions: String,
    pub functions: HashMap<String, FunctionConfig>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(into = "ExcerciseSerialize")]
pub struct Exercise {
    config: ExerciseConfig,
    code: HashMap<Language, String>,
    runners: HashMap<Language, String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum TestResult {
    #[serde(rename = "correct")]
    Correct { stdout: String },
    #[serde(rename = "incorrect")]
    Incorrect { stdout: String },
    #[serde(rename = "exception")]
    Exception { traceback: String, stdout: String },
}

#[derive(Debug, Clone, Serialize)]
pub struct TestResults {
    pub visible: Vec<TestResult>,
    pub hidden: bool,
    pub passed: bool,
}

impl Exercise {
    pub fn new(
        config: ExerciseConfig,
        code: HashMap<Language, String>,
        runners: HashMap<Language, String>,
    ) -> Self {
        Self {
            config,
            code,
            runners,
        }
    }

    pub fn test_code(
        &self,
        lang: &Language,
        content: &str,
        id: &str,
        cfg: &Config,
    ) -> anyhow::Result<HashMap<String, TestResults>> {
        #[derive(Debug, Deserialize)]
        #[serde(tag = "type")]
        enum TestOutput {
            #[serde(rename = "output")]
            Output {
                value: serde_json::Value,
                stdout: String,
            },
            #[serde(rename = "exception")]
            Exception { traceback: String, stdout: String },
        }

        let RunOutput { stdout, stderr, .. } = run(
            cfg.docker.language_config.get(lang.image()).unwrap(),
            &cfg.docker,
            &self.runners.get(lang).with_context(|| {
                format!(
                    "Runner of lang `{}` not found for this question",
                    lang.image()
                )
            })?,
            HashMap::from_iter([(id.to_string(), content.as_bytes())]),
            "",
        )
        .context("While running generator")?;

        let test_cases: HashMap<String, Vec<TestOutput>> = serde_json::from_str(&stdout)
            .with_context(|| {
                format!("While parsing test cases\nstdout:\n{stdout}\n\nstderr:\n{stderr}")
            })?;
        let mut results = HashMap::new();

        for (func, suite) in test_cases {
            let fn_config = &self.config.functions[&func];
            let tests = &fn_config.tests;
            let (visible, hidden) = suite.as_slice().split_at(fn_config.visible_cases as usize);

            let mut visible_passed = true;
            let visible = visible
                .into_iter()
                .enumerate()
                .map(|(i, t)| match t {
                    TestOutput::Output { value, stdout } => {
                        let stdout = stdout.to_string();
                        match value == &tests[i].output {
                            true => TestResult::Correct { stdout },
                            false => {
                                visible_passed = false;
                                TestResult::Incorrect { stdout }
                            }
                        }
                    }
                    TestOutput::Exception { traceback, stdout } => {
                        let traceback = traceback.to_string();
                        let stdout = stdout.to_string();
                        visible_passed = false;
                        TestResult::Exception { traceback, stdout }
                    }
                })
                .collect();
            let hidden = hidden.into_iter().enumerate().all(|(i, t)| match t {
                TestOutput::Output { value, .. } => {
                    &tests[i + fn_config.visible_cases as usize].output == value
                }
                TestOutput::Exception { .. } => false,
            });
            results.insert(
                func,
                TestResults {
                    visible,
                    hidden,
                    passed: visible_passed && hidden,
                },
            );
        }

        Ok(results)
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

pub fn runner_template(lang: &Language, cfg: &ExerciseConfig, id: &str) -> anyhow::Result<String> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file(
            "runner",
            &path::LANGUAGES.join(lang.image()).join("runner.hbs"),
        )
        .context("While registering template file")?;
    handlebars.register_escape_fn(handlebars::no_escape);
    let out = handlebars
        .render(
            "runner",
            &json!({
                "code_file": id,
                "functions": &cfg.functions,
            }),
        )
        .context("While rendering template file")?;
    Ok(out)
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub inputs: Vec<serde_json::Value>,
    #[serde(skip_serializing)]
    pub output: serde_json::Value,
    #[serde(default)]
    #[serde(skip_serializing_if = "bool_is_false")]
    pub hidden: bool,
}

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
    handlebars
        .register_template_file(
            "generator",
            &path::LANGUAGES.join(lang.image()).join("generator.hbs"),
        )
        .context("While registerring template file")?;
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

    let mut test_cases: HashMap<String, Vec<TestCase>> = serde_json::from_str(&stdout)
        .with_context(|| {
            format!("While parsing test cases\nstdout:\n{stdout}\n\nstderr:\n{stderr}")
        })?;

    for (func, mut tests) in test_cases.drain() {
        let cfg = &mut exercise_cfg
            .functions
            .get_mut(&func)
            .with_context(|| format!("ExcerciseConfig does not cotain function {func}"))?;
        anyhow::ensure!(
            tests.len() == (cfg.hidden_cases + cfg.visible_cases) as usize,
            "Test cases for function `{}` < cfg.hidden_cases + cfg.visible_cases",
            func
        );
        for i in cfg.hidden_cases as usize..tests.len() {
            tests[i].hidden = true
        }
        cfg.tests = tests;
    }

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
                    tests: vec![],
                },
            )]),
        };

        generate(
            &Language::Python,
            &config,
            "def gen_test(ctx):\n    ctx.inputs([1])\n    ctx.output(1)\n",
            &mut exercise_cfg,
        )
        .expect("Errors in generation");

        let tests = &exercise_cfg.functions["test"].tests;
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
