use std::{env, fs};

use amplitude_common::{config::Config, config_and_set_path};
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

    let config = config_and_set_path()?;
    parse(&config)?;

    Ok(())
}
