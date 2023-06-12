#![feature(decl_macro)]

use std::{env, fs, path::PathBuf};

use config::{Args, Config};

pub mod config;
pub mod path;

pub fn config_and_set_path() -> anyhow::Result<config::Config> {
    let binding = env::current_dir()?.to_path_buf();
    let dir = binding.components().take_while(|c| c.as_os_str() != "amplitude").collect::<PathBuf>().join("amplitude");
    env::set_current_dir(&dir)?;

    let args = Args::parse();
    let mut config: Config = toml::from_str::<Config>(&fs::read_to_string(&args.config)?)
        .expect("Not a valid config file");
    config.args = args;

    Ok(config)
}
