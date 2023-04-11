use amplitude_common::config;
use amplitude_markdown::parse::parse_dir;

use parking_lot::RwLock;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::MutexGuard;

use amplitude_markdown::state::ParseState;
use rusqlite::Connection;
use serde::Deserialize;
use tracing::info;

use crate::db::Database;

pub struct State {
    db: Mutex<Connection>,
    pub parse_state: RwLock<ParseState>,
    pub config: Config,
}

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,

    pub db_path: String,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let config_file = PathBuf::from(
            env::args()
                .nth(1)
                .unwrap_or_else(|| "./config.toml".to_string()),
        );
        let config = toml::from_str::<Config>(&fs::read_to_string(config_file).unwrap()).unwrap();

        let mut db = Connection::open(&config.db_path).unwrap();
        db.init();
        info!("Loaded database at `{}`", config.db_path);

        let parse_state = parse_dir(&config::INPUT, &config::RENDERED)?;

        Ok(Self {
            db: Mutex::new(db),
            parse_state: RwLock::new(parse_state),
            config,
        })
    }

    pub fn db(&self) -> MutexGuard<Connection> {
        self.db.lock().unwrap()
    }
}
