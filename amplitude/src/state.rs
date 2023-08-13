use std::{fs, path::PathBuf};

use amplitude_common::config::{Args, Config};
use parking_lot::{RwLock, RwLockReadGuard};
use rusqlite::Connection;

use crate::database::Db;

use amplitude_common::path;
use amplitude_markdown::parse::{parse, ParseData};

pub struct State {
    pub db: Db,
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

        let db = Db::new(Connection::open(&path::DATABASE)?);
        db.init()?;

        let parse_data = parse(&config)?;

        Ok(Self {
            db,
            parse_data: RwLock::new(parse_data),
            config,
        })
    }
}
