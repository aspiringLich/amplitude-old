use std::{fs, path::PathBuf};

use amplitude_common::config::{Args, AuthConfig, Config};
use anyhow::Context;
use parking_lot::{RwLock, RwLockReadGuard};
use rusqlite::Connection;

use crate::database::Db;

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
        let mut config: Config =
            toml::from_str(&fs::read_to_string(&args.config).context("While reading config file")?)
                .context("While parsing config file")?;
        let auth: AuthConfig = toml::from_str(
            &fs::read_to_string(&args.auth).context("While reading auth config file")?,
        )
        .context("While parsing auth file")?;

        config.args = args;
        config.auth = auth;

        let tmp_folder = PathBuf::from(&config.docker.tmp_folder);
        if !tmp_folder.exists() {
            fs::create_dir_all(tmp_folder)?;
        }

        fs::create_dir_all(config.server.database_path.parent().unwrap())?;
        let db = Db::new(
            Connection::open(&config.server.database_path)
                .context("While opening connection to Database")?,
        );
        db.init().context("While initializing Database")?;

        let parse_data = parse(&config)?;

        Ok(Self {
            db,
            parse_data: RwLock::new(parse_data),
            config,
        })
    }
}
