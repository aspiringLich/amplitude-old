#![feature(try_trait_v2)]

use afire::{
    trace::{self, Level},
    Server,
};

use std::{path::PathBuf, sync::RwLock};

use amplitude_common::{config, state::State};
use amplitude_markdown::parse::{parse_dir, parse_dir_watch};

use crate::logger::AfireLogger;
mod error;
mod logger;
mod routes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    trace::set_log_formatter(AfireLogger);
    trace::set_log_level(Level::Trace);
    tracing_subscriber::fmt().init();
    // let args = Args::parse();

    let parse_state = parse_dir(&config::INPUT, &config::RENDERED)?;
    dbg!(&parse_state);
    let state = State {
        parse: RwLock::new(parse_state),
    };

    if !PathBuf::from("web/dist").exists() {
        panic!("web/dist not built! please go into web/ and run `npm run build`");
    }

    let mut server = Server::<State>::new("localhost", 8080).state(state);
    let state = server.state.clone().unwrap();
    std::thread::spawn(|| parse_dir_watch(state));

    routes::attach(&mut server);

    server.start_threaded(16).unwrap();

    Ok(())
}
