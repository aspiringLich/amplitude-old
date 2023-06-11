use super::*;

use crate::parse::parse_md;
pub use amplitude_runner::exercise::Exercise;
use amplitude_runner::{exercise::ExerciseConfig, lang::Language};
use std::str::FromStr;

impl Item for Exercise {
    fn parse_from_dir(
        dir: &Path,
        contents: DirContents,
        ctx: &mut DataContext,
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
        ensure!(
            contents
                .query("src/generator", FileType::Code)
                .next()
                .is_some(),
            "src/generator.<code>",
            "Test Case generator"
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
        config.instructions = parse_md(&fs::read_to_string(dir.join("instructions.md"))?, ctx)?;

        Ok(ItemType::Exercise(Exercise::new(config, code)))
    }
}
