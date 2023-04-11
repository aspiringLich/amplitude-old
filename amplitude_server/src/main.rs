#![feature(try_trait_v2)]

use afire::{
    trace::{self, Level},
    Server,
};
use tracing::info;
use watch::parse_dir_watch;

use std::{path::PathBuf, process};

use crate::logger::AfireLogger;
use amplitude_state::{db::Database, State};

mod error;
mod logger;
mod routes;
mod watch;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    trace::set_log_formatter(AfireLogger);
    trace::set_log_level(Level::Trace);
    tracing_subscriber::fmt().init();

    let state = State::new()?;

    if !PathBuf::from("web/dist").exists() {
        panic!("web/dist not built! please go into web/ and run `npm run build`");
    }

    let mut server = Server::<State>::new(&state.config.host, state.config.port).state(state);
    routes::attach(&mut server);

    let state = server.state.clone().unwrap();
    std::thread::spawn(|| parse_dir_watch(state));

    let app = server.app();
    ctrlc::set_handler(move || {
        info!("Exiting");
        app.db().cleanup();
        process::exit(0);
    })
    .unwrap();

    server.start_threaded(16).unwrap();
    Ok(())
}
