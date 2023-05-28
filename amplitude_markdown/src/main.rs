use std::{env, fs};

use amplitude_common::config::Config;
use amplitude_markdown::parse::parse;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

fn main() -> anyhow::Result<()> {
    let filter = filter::Targets::new().with_default(LevelFilter::DEBUG);
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    if env::current_dir()?.ends_with("amplitude_markdown") {
        env::set_current_dir("../")?;
    }
    let config = toml::from_str::<Config>(&fs::read_to_string("config.toml")?)?;
    parse(&config)?;

    Ok(())
}
