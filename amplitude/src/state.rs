use std::{fs, path::PathBuf};

use amplitude_common::config::{Args, Config};
use parking_lot::{Mutex, MutexGuard, RwLock, RwLockReadGuard};
use rusqlite::Connection;

use crate::database::Database;

use amplitude_common::path;
use amplitude_markdown::parse::{parse, ParseData};

pub struct State {
    db: Mutex<Connection>,
    pub parse_data: RwLock<ParseData>,
    pub config: Config,
}

impl State {
    pub fn parse_data(&self) -> RwLockReadGuard<ParseData> {
        self.parse_data.read()
    }

    pub fn new() -> anyhow::Result<Self> {
        let args = Args::parse();
        let mut config = toml::from_str::<Config>(&fs::read_to_string(&args.config)?)?;
        config.args = args;

        let tmp_folder = PathBuf::from(&config.docker.tmp_folder);
        if !tmp_folder.exists() {
            fs::create_dir_all(tmp_folder)?;
        }

        let mut db = Connection::open(&path::DATABASE)?;
        db.init()?;

        let parse_data = parse(&config)?;

        Ok(Self {
            db: Mutex::new(db),
            parse_data: RwLock::new(parse_data),
            config,
        })
    }

    pub fn db(&self) -> MutexGuard<Connection> {
        self.db.lock()
    }
}
