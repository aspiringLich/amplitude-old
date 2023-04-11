#![feature(try_trait_v2)]

use std::{path::PathBuf, process};

use afire::{
    trace::{self, Level},
    Middleware, Server,
};
use logger::RequestLogger;
use state::State;
use tracing::{info, warn};
use watch::parse_dir_watch;

use crate::{database::Database, logger::AfireLogger};
mod database;
mod error;
mod logger;
mod misc;
mod routes;
mod session;
mod state;
mod watch;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    trace::set_log_formatter(AfireLogger);
    trace::set_log_level(Level::Trace);
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    if !PathBuf::from("web/dist").exists() {
        warn!("web/dist not built!");
        warn!("^ please go into web/ and run `npm run build`");
    }

    let state = State::new()?;
    let mut server = Server::<State>::new(&state.config.host, state.config.port).state(state);
    RequestLogger.attach(&mut server);
    routes::attach(&mut server);

    let state = server.state.clone().unwrap();
    std::thread::spawn(|| parse_dir_watch(state));

    let app = server.app();
    let threads = app.config.threads;
    ctrlc::set_handler(move || {
        info!("Exiting");
        app.db().cleanup();
        process::exit(0);
    })
    .unwrap();

    server.start_threaded(threads).unwrap();
    Ok(())
}
