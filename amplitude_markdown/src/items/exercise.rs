use super::*;

use crate::parse::parse_md;
pub use amplitude_runner::exercise::Exercise;
use amplitude_runner::{
    exercise::{generate, runner_template, ExerciseConfig, LanguageInfo},
    lang::Language,
};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    str::FromStr,
};

impl Item for Exercise {
    fn parse_from_dir(
        dir: &Path,
        contents: DirContents,
        ctx: &mut DataContext,
        cfg: &Config,
    ) -> anyhow::Result<ItemType> {
        ensure!(contents.contains("instructions.md"), "instructions.md");
        ensure!(contents.contains("config.toml"), "config.toml");
        ensure!(
            contents.query("src", FileType::Directory).next().is_some(),
            "src/",
            "Source directory"
        );

        let id = ctx.id().rsplit('/').next().unwrap().to_string();
        let binding = "src/".to_string() + &id;
        let mut starting_code = contents.query(&binding, FileType::Code).peekable();
        ensure!(
            starting_code.peek().is_some(),
            format!("src/{}.<code>", id),
            "Starting code"
        );
        let generator = contents
            .query("src/generator", FileType::Code)
            .collect::<Vec<_>>();
        ensure!(
            generator.len() > 0,
            "src/generator.<code>",
            "Test Case generator"
        );
        anyhow::ensure!(
            generator.len() == 1,
            "Multiple test case generator (`src/generator.<code>`) files found! Only one is allowed."
        );

        let config =
            &fs::read_to_string(dir.join("config.toml")).context("While reading `config.toml`")?;
        let mut config: ExerciseConfig =
            toml::from_str(config).context("While parsing `config.toml`")?;

        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        // set seed of function configs
        for (func, cfg) in config.functions.iter_mut() {
            let mut h = hasher.clone();
            func.hash(&mut h);
            cfg.seed = h.finish();
        }

        let lang = Language::from_str(&generator[0].ext)?;
        let content = fs::read_to_string(generator[0].path(dir))
            .context("While reading test case generator file")?;

        generate(&lang, &cfg, &content, &mut config).context("While generating test cases")?;

        let iter = starting_code
            .filter_map(|item| Language::from_str(&item.ext).ok().map(|x| (item, x)))
            .map(|(item, lang)| -> anyhow::Result<_> {
                Ok((
                    lang,
                    LanguageInfo {
                        code: fs::read_to_string(item.path(dir)).context("Expected valid path")?,
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

        config.instructions = parse_md(
            &fs::read_to_string(dir.join("instructions.md"))
                .context("While reading `instructions.md`")?,
            ctx,
        )
        .context("While parsing markdown for `instructions.md`")?;

        Ok(ItemType::Exercise(Exercise::new(config, lang_info)))
    }
}
