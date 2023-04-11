use amplitude_common::config;
use amplitude_markdown::parse::parse_dir;

use parking_lot::{Mutex, MutexGuard, RwLock};
use std::{env, fs, path::PathBuf};

use amplitude_markdown::state::ParseState;
use rusqlite::Connection;
use serde::Deserialize;
use tracing::info;

use crate::db::Database;

pub struct State {
    db: Mutex<Connection>,
    // breon this is not a nice name
    pub parse_state: RwLock<ParseState>,
    pub config: Config,
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
        self.db.lock()
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_path: String,

    pub google_oauth: Option<GoogleOauth>,
    pub github_oauth: Option<GithubOauth>,
}

#[derive(Deserialize)]
pub struct GoogleOauth {
    pub client_id: String,
    pub client_secret: String,
    pub external_url: String,
}

#[derive(Deserialize)]
pub struct GithubOauth {
    pub github_app_id: String,
    pub github_app_secret: String,
    pub ext_url: String,
}
