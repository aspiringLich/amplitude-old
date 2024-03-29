use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use clap::Parser;
use serde::Deserialize;

use crate::path;

#[derive(Parser, Default, Deserialize, Debug)]
pub struct Args {
    /// Whether or not to pull the repo from github or to use the existing one
    #[arg(long, default_value_t = false)]
    pub pull: bool,
    /// The path of the config file
    #[arg(long, default_value = "config.toml")]
    pub config: PathBuf,
    /// The path of the auth file
    #[arg(long, default_value = "auth.toml")]
    pub auth: PathBuf,
}

impl Args {
    /// call `clap::Parser::parse()`
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ParseConfig {
    pub git_url: String,
    pub clone_path: String,
    pub asset_path: String,
    pub asset_prefix: String,
    pub image_extensions: HashSet<String>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub threads: usize,
    pub req_duration: u64,
    pub database_path: PathBuf,
}

#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
pub struct AuthConfig {
    pub google_oauth: Option<GoogleOauth>,
    pub github_oauth: Option<GithubOauth>,
    pub bot: Option<GithubBotAuth>,
}

fn args() -> Args {
    Args::parse()
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub server: ServerConfig,
    pub docker: DockerConfig,
    #[serde(default)]
    pub auth: AuthConfig,
    pub parse: ParseConfig,
    #[serde(default = "args")]
    pub args: Args,
}

fn language_config() -> HashMap<String, LanguageConfig> {
    toml::from_str(
        fs::read_to_string(path::LANGUAGES.join("languages.toml"))
            .unwrap()
            .as_str(),
    )
    .unwrap()
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DockerConfig {
    pub tmp_folder: String,
    pub command: String,
    pub timeout: u64,
    #[serde(default = "language_config")]
    pub language_config: HashMap<String, LanguageConfig>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GoogleOauth {
    pub client_id: String,
    pub client_secret: String,
    pub external_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GithubOauth {
    pub app_id: String,
    pub app_secret: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubBotAuth {
    pub app_id: String,
    pub app_secret: String,
    pub installation_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct LanguageConfig {
    pub image_name: String,
    pub source_path: String,
    pub extension: String,
    pub delimiter: String,
}
