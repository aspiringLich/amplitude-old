use std::{fs, path::PathBuf};

use amplitude_common::config::{Args, Config};
use parking_lot::{Mutex, MutexGuard};
use rusqlite::Connection;

use crate::database::Database;

use amplitude_common::path;
use amplitude_markdown::parse::{parse, ParseData};

pub struct State {
    db: Mutex<Connection>,
    pub parse_data: ParseData,
    pub config: Config,
}

impl State {
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

        let parse_state = parse(&config)?;

        Ok(Self {
            db: Mutex::new(db),
            parse_data: parse_state,
            config,
        })
    }

    pub fn db(&self) -> MutexGuard<Connection> {
        self.db.lock()
    }
}
