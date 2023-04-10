#![feature(try_trait_v2)]

use afire::{
    trace::{self, Level},
    Server,
};
use amplitude_markdown::parse::parse_dir_watch;

use std::path::PathBuf;

use crate::logger::AfireLogger;
use app::App;
mod app;
mod db;
mod error;
mod logger;
mod routes;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    trace::set_log_formatter(AfireLogger);
    trace::set_log_level(Level::Trace);
    tracing_subscriber::fmt().init();

    let app = App::new()?;

    if !PathBuf::from("web/dist").exists() {
        panic!("web/dist not built! please go into web/ and run `npm run build`");
    }

    let mut server = Server::<App>::new(&app.config.host, app.config.port).state(app);
    let docs = server.app().documents.clone();
    std::thread::spawn(|| parse_dir_watch(docs));

    routes::attach(&mut server);

    server.start_threaded(16).unwrap();
    Ok(())
}
