use super::*;

use crate::parse::parse_md;
pub use amplitude_runner::exercise::Exercise;
use amplitude_runner::{
    exercise::{generate, runner_template, ExerciseConfig, LanguageInfo},
    lang::Language,
};
use std::{
    collections::HashMap,
    str::FromStr,
};

impl FromDirectory for Exercise {
    fn from_directory(
        content: &DirectoryContent,
        context: &mut DataContext,
        cfg: &Config,
    ) -> anyhow::Result<Self> {
        let instructions = content.query_file("instructions", FileType::Markdown)?;
        let config = content.query_file("config", FileType::Toml)?;
        let src_path = content.get_directory("src")?;
        let src = DirectoryContent::new(&src_path)?;

        let id = context.id().rsplit_once('/').unwrap().1.to_string();
        let starting_code = src.query_files(&id, FileType::Code)?;
        let generator = src
            .query_files("generator", FileType::Code)?
            .collect::<Vec<_>>();

        let mut config: ExerciseConfig =
            toml::from_str(&config.read_to_string()?).context("While parsing `config.toml`")?;

        config
            .functions
            .values_mut()
            .for_each(|f| f.seed = context.next_seed());

        let lang = Language::from_str(&generator[0].ext)?;
        let content = fs::read_to_string(generator[0].path())
            .context("While reading test case generator file")?;

        generate(&lang, cfg, &content, &mut config).context("While generating test cases")?;

        let iter = starting_code
            .filter_map(|item| Language::from_str(&item.ext).ok().map(|x| (item, x)))
            .map(|(item, lang)| -> anyhow::Result<_> {
                Ok((
                    lang,
                    LanguageInfo {
                        code: fs::read_to_string(item.path()).context("Expected valid path")?,
                        runner: runner_template(&lang, &config, &id)
                            .context("While generating runner template")?,
                    },
                ))
            });

        let mut lang_info = HashMap::new();
        for info in iter {
            match info {
                Ok((lang, info)) => {
                    lang_info.insert(lang, info);
                }
                Err(e) => {
                    return Err(e.context("While parsing Language Info"));
                }
            }
        }

        config.instructions = parse_md(&instructions.read_to_string()?, context)
            .context("While parsing markdown for `instructions.md`")?;

        Ok(Exercise::new(config, lang_info))
    }
}

pub fn transform(exercise: &mut Exercise) {
    for (_, cfg) in exercise.config.functions.iter_mut() {
        cfg.tests.retain(|test| !test.hidden);
    }
}
