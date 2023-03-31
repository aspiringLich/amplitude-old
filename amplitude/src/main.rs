#![feature(try_trait_v2)]

use afire::{
    extension::ServeStatic,
    trace::{self, Level},
    Content, Middleware, Response, Server,
};

use clap::Parser;
use std::{fs::File, path::PathBuf, sync::Mutex};

use amplitude_common::{config, Args};
use amplitude_markdown::parse::{parse_dir, parse_dir_watch, ParseState};
mod error;
mod routes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    trace::set_log_level(Level::Trace);
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let state = parse_dir(&config::INPUT, &config::OUTPUT)?;

    if !PathBuf::from("web/dist").exists() {
        panic!("web/dist not built! please go into web/ and run `npm run build`");
    }

    if args.watch {
        let state = Mutex::new(state);
        let server: Server<Mutex<ParseState>> = Server::new("localhost", 8080).state(state);
        let state = server.state.clone().unwrap();
        std::thread::spawn(|| parse_dir_watch(state));
        server.start_threaded(16).unwrap();
    } else {
        let mut server: Server<ParseState> = Server::new("localhost", 8080).state(state);
        routes::attach(&mut server);
        server.start_threaded(16).unwrap();
    }

    Ok(())
}
