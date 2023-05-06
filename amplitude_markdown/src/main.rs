use std::{fs, env};

use amplitude_common::{config::Config, path};
use amplitude_markdown::parse::{clone_repo, parse_dir};

fn main() -> anyhow::Result<()> {
    let config = toml::from_str::<Config>(&fs::read_to_string("../config.toml")?)?;
    if env::current_dir()?.ends_with("amplitude_markdown") {
        env::set_current_dir("../")?;
    }
    clone_repo(&config)?;
    parse_dir(&config)?;
    
    Ok(())
}
