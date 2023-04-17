use serde::Deserialize;

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
