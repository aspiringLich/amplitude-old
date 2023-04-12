use super::*;

const fn true_fn() -> bool {
    true
}
#[derive(Debug, Deserialize)]
pub struct RawCourseConfig {
    pub title: String,
    pub description: String,
    #[serde(default = "true_fn")]
    pub readable: bool,
}

#[derive(Debug, Deserialize)]
pub struct RawDirConfig {
    pub title: String,
    #[serde(default = "true_fn")]
    pub readable: bool,
}
