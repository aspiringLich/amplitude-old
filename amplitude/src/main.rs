#![feature(try_trait_v2)]

use amplitude_common::{config, Args};
use amplitude_markdown::parse::{parse_dir, parse_dir_watch};
use clap::Parser;

mod routes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    parse_dir(&config::INPUT, &config::OUTPUT)?;
    if args.watch {
        std::thread::spawn(parse_dir_watch);
    }

    let mut server = afire::Server::new("localhost", 8080);
    routes::attach(&mut server);

    server.start_threaded(16).unwrap();

    Ok(())
}
