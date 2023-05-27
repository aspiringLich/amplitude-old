use super::*;

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawArticleConfig {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct ArticleConfig {
    pub id: String,
    pub name: String,
    pub description: String,
}
