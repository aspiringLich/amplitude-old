#![feature(try_trait_v2)]

use afire::{
    extension::ServeStatic,
    trace::{self, Level},
    Content, Middleware, Response, Server,
};

use clap::Parser;
use std::{fs::File, path::PathBuf};

use amplitude_common::{config, Args};
use amplitude_markdown::parse::{parse_dir, parse_dir_watch};
mod routes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    trace::set_log_level(Level::Trace);
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    parse_dir(&config::INPUT, &config::OUTPUT)?;
    if args.watch {
        std::thread::spawn(|| parse_dir_watch());
    }

    if !PathBuf::from("web/dist").exists() {
        panic!("[-] Web dist not built!");
    }

    let mut server = Server::new("localhost", 8080);
    ServeStatic::new("web/dist")
        .not_found(|_req, _dis| {
            Response::new()
                .stream(File::open("web/dist/index.html").expect("Webpage not built"))
                .content(Content::HTML)
        })
        .attach(&mut server);
    routes::attach(&mut server);

    server.start_threaded(16).unwrap();

    Ok(())
}
