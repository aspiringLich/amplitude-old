#![feature(try_trait_v2)]

use std::{path::PathBuf, process};

use afire::{
    trace::{self, Level},
    Middleware, Server,
};
use logger::RequestLogger;
use state::State;
use tracing::{info, metadata::LevelFilter, warn};
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

use crate::{database::Database, logger::AfireLogger};
mod database;
mod error;
mod logger;
mod misc;
mod routes;
mod runner;
mod session;
mod state;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    trace::set_log_formatter(AfireLogger);
    trace::set_log_level(Level::Trace);
    let filter = filter::Targets::new()
        .with_default(LevelFilter::INFO)
        .with_target("afire", LevelFilter::TRACE)
        .with_target("amplitude", LevelFilter::TRACE);
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    if !PathBuf::from("web/dist").exists() {
        warn!("web/dist not built!");
        warn!("^ please go into web/ and run `npm run build`");
    }

    let state = State::new()?;
    let mut server =
        Server::<State>::new(&state.config.server.host, state.config.server.port).state(state);
    RequestLogger.attach(&mut server);
    routes::attach(&mut server);

    let app = server.app();
    let threads = app.config.server.threads;
    ctrlc::set_handler(move || {
        info!("Exiting");
        app.db().cleanup().unwrap();
        process::exit(0);
    })
    .unwrap();

    server.start_threaded(threads).unwrap();
    Ok(())
}
