use std::{collections::HashMap, str::FromStr};

use amplitude_common::lang::Language;

use crate::var_types::VariableType;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FunctionConfig {
    inputs: Vec<VariableType>,
    output: VariableType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExerciseConfig {
    #[serde(flatten)]
    functions: HashMap<String, FunctionConfig>,
}

#[derive(Debug, Serialize)]
pub struct Exercise {
    config: ExerciseConfig,
    code: HashMap<Language, String>,
}

impl Exercise {
    pub fn from_raw(contents: &DirContents, path: &Path) -> anyhow::Result<Self> {
        let config =
            &fs::read_to_string(path.join("config.toml")).context("While reading `config.toml`")?;

        let code = contents
            .query("start", FileType::Code)
            .map(|item| {
                (
                    Language::from_str(&item.ext)
                        .expect("Already guaranteed by check in Item impl of Exercise"),
                    fs::read_to_string(item.path(path)).expect("guaranteed valid path"),
                )
            })
            .collect();

        Ok(Self {
            config: toml::from_str(config).context("While parsing `config.toml`")?,
            code,
        })
    }
}

impl Item for Exercise {
    fn parse_from_dir(
        dir: &Path,
        contents: DirContents,
        _context: &mut DataContext,
    ) -> anyhow::Result<ItemType> {
        ensure!(
            contents.query("gen", FileType::Code).next().is_some(),
            "gen.<code>",
            "Test case generator"
        );
        ensure!(
            contents.query("start", FileType::Code).next().is_some(),
            "start.<code>",
            "Starting code"
        );
        ensure!(contents.contains("instructions.md"), "instructions.md");
        ensure!(contents.contains("config.toml"), "config.toml");

        let exercise = Exercise::from_raw(&contents, dir)?;
        Ok(ItemType::Exercise(exercise))
    }
}

mod tests {}
