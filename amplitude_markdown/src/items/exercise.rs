use super::*;

use crate::parse::parse_md;
pub use amplitude_runner::exercise::Exercise;
use amplitude_runner::{
    exercise::{generate, ExerciseConfig},
    lang::Language,
};
use std::str::FromStr;

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
        ensure!(
            contents
                .query(&("src/".to_string() + &id), FileType::Code)
                .next()
                .is_some(),
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

        let code = contents
            .query(&id, FileType::Code)
            .map(|item| {
                (
                    Language::from_str(&item.ext)
                        .expect("Already guaranteed by check in Item impl of Exercise"),
                    fs::read_to_string(item.path(dir)).expect("guaranteed valid path"),
                )
            })
            .collect();
        config.instructions = parse_md(
            &fs::read_to_string(dir.join("instructions.md"))
                .context("While reading `instructions.md`")?,
            ctx,
        )
        .context("While parsing markdown for `instructions.md`")?;

        let lang = Language::from_str(&generator[0].ext)?;
        let content = fs::read_to_string(generator[0].path(dir))?;

        generate(&lang, &cfg, &content, &mut config).context("While generating test cases")?;

        Ok(ItemType::Exercise(Exercise::new(config, code)))
    }
}
