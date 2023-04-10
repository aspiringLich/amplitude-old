use std::{env, fs, path::PathBuf, sync::Arc};

use amplitude_common::{config, state::ParseState};
use amplitude_markdown::parse::parse_dir;
use anyhow::Result;
use parking_lot::{Mutex, MutexGuard, RwLock};
use rusqlite::Connection;
use serde::Deserialize;
use tracing::info;

use crate::db::Database;

pub struct App {
    db: Mutex<Connection>,
    pub documents: Arc<RwLock<ParseState>>,
    pub config: Config,
}

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,

    pub db_path: String,
}

impl App {
    pub fn new() -> Result<Self> {
        let config_file = PathBuf::from(
            env::args()
                .nth(1)
                .unwrap_or_else(|| "./config.toml".to_string()),
        );
        let config = toml::from_str::<Config>(&fs::read_to_string(config_file).unwrap()).unwrap();

        let db = Connection::open(&config.db_path).unwrap();
        db.init();
        info!("Loaded database at `{}`", config.db_path);

        let parse_state = parse_dir(&config::INPUT, &config::RENDERED)?;

        Ok(Self {
            db: Mutex::new(db),
            documents: Arc::new(RwLock::new(parse_state)),
            config,
        })
    }

    pub fn db(&self) -> MutexGuard<Connection> {
        self.db.lock()
    }
}
