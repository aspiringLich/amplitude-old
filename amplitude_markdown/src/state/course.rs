use super::*;

#[derive(Debug, Deserialize)]
pub struct TrackRaw {
    pub name: String,
    pub path: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct CourseConfigRaw {
    pub title: String,
    pub tracks: Vec<TrackRaw>,
}

#[derive(Debug, Default)]
pub struct Track {
    pub name: String,
    pub description: String,
    pub files: Vec<FileEntry>,
}

#[derive(Debug, Default)]
pub struct CourseConfig {
    pub tracks: Vec<Track>,
}

pub fn parse_course_config(cfg: &str) -> anyhow::Result<CourseConfig> {
    let config: CourseConfigRaw = toml::from_str(cfg)?;

    todo!()
}
