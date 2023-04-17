use std::{env, fs, path::PathBuf};

use amplitude_common::config::Config;
use parking_lot::{Mutex, MutexGuard, RwLock};
use rusqlite::Connection;
use tracing::info;

use crate::database::Database;
use amplitude_common::path;
use amplitude_markdown::parse::parse_dir;
use amplitude_markdown::state::ParseState;

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

        let parse_state = parse_dir(&path::INPUT, &path::RENDERED)?;

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
