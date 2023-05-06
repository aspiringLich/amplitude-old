use std::{env, fs, path::PathBuf};

use amplitude_common::config::{Config, LanguageConfig};
use parking_lot::{Mutex, MutexGuard, RwLock};
use rusqlite::Connection;

use crate::database::Database;
use amplitude_common::path;
use amplitude_markdown::{state::ParseState, parse::parse_dir};

pub struct State {
    db: Mutex<Connection>,
    // breon this is not a nice name
    pub parse_state: RwLock<ParseState>,
    pub language_config: Vec<LanguageConfig>,
    pub config: Config,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let config_file = PathBuf::from(
            env::args()
                .nth(1)
                .unwrap_or_else(|| "./config.toml".to_string()),
        );
        let config = toml::from_str::<Config>(&fs::read_to_string(config_file)?)?;

        let tmp_folder = PathBuf::from(&config.docker.tmp_folder);
        if !tmp_folder.exists() {
            fs::create_dir_all(tmp_folder)?;
        }

        let mut db = Connection::open(&config.db_path)?;
        db.init()?;

        let parse_state = parse_dir(&config.parse_config)?;

        let raw_lang_config = fs::read_to_string("./langs/languages.json")?;

        Ok(Self {
            db: Mutex::new(db),
            parse_state: RwLock::new(parse_state),
            language_config: serde_json::from_str(&raw_lang_config)?,
            config,
        })
    }

    pub fn db(&self) -> MutexGuard<Connection> {
        self.db.lock()
    }
}
