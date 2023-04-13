use std::{env, fs, path::PathBuf};

use parking_lot::{Mutex, MutexGuard, RwLock};
use rusqlite::Connection;
use serde::Deserialize;

use crate::database::Database;
use amplitude_common::config;
use amplitude_markdown::parse::parse_dir;
use amplitude_markdown::state::ParseState;

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

        let parse_state = parse_dir(&config::INPUT, &config::RENDERED)?;

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

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub threads: usize,
    pub db_path: String,
    pub req_duration: u64,

    pub docker: Docker,
    pub google_oauth: Option<GoogleOauth>,
    pub github_oauth: Option<GithubOauth>,
}

#[derive(Deserialize)]
pub struct Docker {
    pub tmp_folder: String,
    pub command: String,
    pub timeout: u64,
}

#[derive(Deserialize)]
pub struct GoogleOauth {
    pub client_id: String,
    pub client_secret: String,
    pub external_url: String,
}

#[derive(Deserialize)]
pub struct GithubOauth {
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageConfig {
    pub name: String,
    pub path: String,
    pub image_name: String,
    pub source_path: String,
}

pub trait GetLang {
    fn get_lang(&self, lang: &str) -> Option<&LanguageConfig>;
}

impl GetLang for Vec<LanguageConfig> {
    fn get_lang(&self, lang: &str) -> Option<&LanguageConfig> {
        self.iter().find(|x| x.name == lang)
    }
}
