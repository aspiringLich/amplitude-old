use std::default::default;

use crate::util::get_path_components;

use super::{article::ArticleConfig, *};

const fn true_fn() -> bool {
    true
}

pub type Children = Vec<ChildEntry>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ChildEntry {
    #[serde(skip_deserializing)]
    pub children: Children,
    #[serde(skip_deserializing)]
    pub id: String,
    pub title: String,
    pub readable: bool,
    #[serde(skip_serializing)]
    pub ord: usize,
}

impl From<RawCourseConfig> for ChildEntry {
    fn from(value: RawCourseConfig) -> Self {
        Self {
            id: default(),
            title: value.title,
            readable: value.readable,
            children: default(),
            ord: default(),
        }
    }
}

impl From<TrackConfig> for ChildEntry {
    fn from(value: TrackConfig) -> Self {
        Self {
            id: default(),
            title: value.title,
            readable: value.readable,
            children: value.children,
            ord: value.ord,
        }
    }
}

impl From<ArticleConfig> for ChildEntry {
    fn from(value: ArticleConfig) -> Self {
        Self {
            id: default(),
            title: value.title,
            readable: true,
            children: default(),
            ord: value.ord,
        }
    }
}

impl ChildEntry {
    pub fn new(title: String, readable: bool) -> Self {
        Self {
            id: default(),
            title,
            readable,
            children: default(),
            ord: default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RawCourseConfig {
    pub title: String,
    pub description: String,
    #[serde(default = "true_fn")]
    pub readable: bool,
}

#[derive(Debug, Serialize)]
pub struct CourseConfig {
    pub title: String,
    pub description: String,
    pub readable: bool,
    #[serde(skip)]
    pub children: HashMap<String, TrackConfig>,
}

impl From<RawCourseConfig> for CourseConfig {
    fn from(value: RawCourseConfig) -> Self {
        Self {
            title: value.title,
            description: value.description,
            readable: value.readable,
            children: default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RawDirConfig {
    pub title: String,
    #[serde(default = "true_fn")]
    pub readable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackConfig {
    pub title: String,
    pub description: String,
    pub readable: bool,
    #[serde(skip_deserializing)]
    pub children: Children,
    #[serde(skip_serializing)]
    pub ord: usize,
}

impl ParseState {
    pub fn insert_course_config(
        &mut self,
        path: String,
        config: CourseConfig,
    ) -> Option<CourseConfig> {
        self.course_config.insert(path, config)
    }

    pub fn get_course_config(&self, path: &str) -> Option<&CourseConfig> {
        self.course_config.get(path)
    }

    pub fn insert_track_config(
        &mut self,
        path: &Path,
        mut config: TrackConfig,
    ) -> anyhow::Result<()> {
        let mut components = get_path_components(path);
        let course_name = components.next().context("path too short")?;
        let track_name = components.next().context("path too short")?;

        config.description = self.parse(&config.description, None)?;

        let course = self.course_config.get_mut(&course_name).with_context(|| {
            format!("Expected to find course `{course_name}` in `course_config`")
        })?;
        course.children.insert(track_name, config);
        Ok(())
    }
}
