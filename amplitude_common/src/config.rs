use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub threads: usize,
    pub db_path: String,
    pub req_duration: u64,

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
    pub app_id: String,
    pub app_secret: String,
}
