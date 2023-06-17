use crate::runner::{run, RunOutput};
use crate::{lang::Language, var_type::VariableType};
use amplitude_common::config::Config;
use amplitude_common::path;

use anyhow::Context;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
    pub inputs: Vec<VariableType>,
    pub output: VariableType,
    #[serde(skip)]
    pub seed: u64,
    #[serde(default = "hidden_cases_default")]
    pub hidden_cases: u32,
    #[serde(default = "visible_cases_default")]
    pub visible_cases: u32,
    #[serde(skip_deserializing)]
    pub tests: Vec<TestCase>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExerciseConfig {
    pub title: String,
    #[serde(skip_deserializing)]
    pub instructions: String,
    pub functions: HashMap<String, FunctionConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Exercise {
    pub config: ExerciseConfig,
    pub lang_info: HashMap<Language, LanguageInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LanguageInfo {
    pub code: String,
    #[serde(skip_serializing)]
    pub runner: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum TestResult {
    #[serde(rename = "correct")]
    Correct { stdout: String },
    #[serde(rename = "incorrect")]
    Incorrect { stdout: String, output: Value },
    #[serde(rename = "error")]
    Error { traceback: String, stdout: String },
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TestResults {
    pub results: Vec<TestResult>,
    pub hidden: bool,
    pub passed: bool,
}

impl Exercise {
    pub fn new(config: ExerciseConfig, language_info: HashMap<Language, LanguageInfo>) -> Self {
        Self {
            config,
            lang_info: language_info,
        }
    }

    pub fn run_tests(
        &self,
        lang: &Language,
        content: &str,
        id: &str,
        cfg: &Config,
    ) -> anyhow::Result<HashMap<String, TestResults>> {
        #[derive(Debug, Deserialize)]
        #[serde(tag = "type")]
        enum TestOutput {
            #[serde(rename = "answer")]
            Answer {
                value: serde_json::Value,
                stdout: String,
            },
            #[serde(rename = "exception")]
            Exception { traceback: String, stdout: String },
        }
        let runner = &self
            .lang_info
            .get(lang)
            .with_context(|| format!("Language `{}` not found for this question", lang.image()))?
            .runner;
        let RunOutput { stdout, stderr, .. } = run(
            cfg.docker.language_config.get(lang.image()).unwrap(),
            &cfg.docker,
            runner,
            HashMap::from_iter([(id.to_string() + ".py", content.as_bytes())]),
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
                    TestOutput::Answer { value, stdout } => {
                        let stdout = stdout.to_string();
                        // dbg!(value, &tests[i].output);
                        match value == &tests[i].output {
                            true => TestResult::Correct { stdout },
                            false => {
                                visible_passed = false;
                                TestResult::Incorrect { stdout, output: value.clone() }
                            }
                        }
                    }
                    TestOutput::Exception { traceback, stdout } => {
                        let traceback = traceback.to_string();
                        let stdout = stdout.to_string();
                        visible_passed = false;
                        TestResult::Error { traceback, stdout }
                    }
                })
                .collect();
            let hidden = hidden.into_iter().enumerate().all(|(i, t)| match t {
                TestOutput::Answer { value, .. } => {
                    &tests[i + fn_config.visible_cases as usize].output == value
                }
                TestOutput::Exception { .. } => false,
            });
            results.insert(
                func,
                TestResults {
                    results: visible,
                    hidden,
                    passed: visible_passed && hidden,
                },
            );
        }

        Ok(results)
    }
}

// impl Into<ExcerciseSerialize> for Exercise {
//     fn into(self) -> ExcerciseSerialize {
//         ExcerciseSerialize {
//             title: self.config.title,
//             instructions: self.config.instructions,
//             code: self.code,
//         }
//     }
// }

pub fn runner_template(lang: &Language, cfg: &ExerciseConfig, id: &str) -> anyhow::Result<String> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file(
            "runner",
            &path::LANGUAGES.join(lang.image()).join("runner.hbs"),
        )
        .context("While registering template file")?;
    handlebars.register_escape_fn(handlebars::no_escape);
    handlebars.set_strict_mode(true);
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
    pub output: serde_json::Value,
    #[serde(default)]
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
    fn test_simple_langs() {
        test_simple(&Language::Python, "def test(x):\n    return x - 1\n").unwrap();
    }

    fn test_simple(lang: &Language, code: &str) -> anyhow::Result<()> {
        let cfg = config_and_set_path().unwrap();

        let config = ExerciseConfig {
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
                    tests: vec![
                        TestCase {
                            inputs: vec![json!(1)],
                            output: json!(0),
                            hidden: false,
                        },
                        TestCase {
                            inputs: vec![json!(3)],
                            output: json!(2),
                            hidden: false,
                        },
                        TestCase {
                            inputs: vec![json!(4)],
                            output: json!(3),
                            hidden: true,
                        },
                        TestCase {
                            inputs: vec![json!(5)],
                            output: json!(4),
                            hidden: true,
                        },
                    ],
                },
            )]),
        };

        let exercise = Exercise {
            lang_info: HashMap::from_iter([(
                lang.clone(),
                LanguageInfo {
                    runner: runner_template(lang, &config, "test").unwrap(),
                    code: String::new(),
                },
            )]),
            config,
        };
        let result = exercise.run_tests(lang, code, "test", &cfg)?;
        anyhow::ensure!(
            result["test"]
                == TestResults {
                    results: vec![
                        TestResult::Correct {
                            stdout: "".to_string()
                        },
                        TestResult::Correct {
                            stdout: "".to_string()
                        }
                    ],
                    hidden: true,
                    passed: true
                }
        );
        Ok(())
    }

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
