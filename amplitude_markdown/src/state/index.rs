use std::default::default;

use serde::Serializer;

use crate::util::get_path_components;

use super::*;

const fn true_fn() -> bool {
    true
}

pub type Children = HashMap<String, ChildEntry>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ChildEntry {
    pub title: String,
    pub readable: bool,
    pub children: Children,
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

impl CourseConfig {
    pub fn from_raw(raw: RawCourseConfig) -> Self {
        let RawCourseConfig {
            title,
            description,
            readable,
        } = raw;
        Self {
            title,
            description,
            readable,
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

fn serialize_children<S: Serializer>(children: &Children, s: S) -> Result<S::Ok, S::Error> {
    children.keys().collect::<Vec<_>>().serialize(s)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrackConfig {
    pub title: String,
    pub description: String,
    pub readable: bool,
    #[serde(serialize_with = "serialize_children", skip_deserializing)]
    pub children: Children,
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
