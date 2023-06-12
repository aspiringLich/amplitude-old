#![feature(decl_macro)]

use std::{env, fs};

use config::{Args, Config};

pub mod config;
pub mod path;

pub fn config_and_set_path() -> anyhow::Result<config::Config> {
    // uhhh i have no idea why i need this, i assume some random rust test fuckery,
    // but this might break
    dbg!(env::current_dir()?);
    let binding = env::current_dir()?;
    
    let dir = binding.file_name().unwrap().to_string_lossy();
    if !dir.starts_with("amplitude") {
        panic!("Boi where da hell are you runnin this from, the folder name doesn't start with `amplitude`")
    } else if dir != "amplitude" {
        env::set_current_dir("../")?;
    }

    let args = Args::parse();
    let mut config: Config = toml::from_str::<Config>(&fs::read_to_string(&args.config)?)
        .expect("Not a valid config file");
    config.args = args;

    Ok(config)
}
