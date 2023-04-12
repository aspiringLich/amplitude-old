use super::*;

const fn true_fn() -> bool {
    true
}

#[derive(Debug, Serialize)]
pub struct ChildEntry {
    pub path: PathBuf,
    pub children: Vec<ChildEntry>,
    pub readable: bool,
}

impl ChildEntry {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            children: vec![],
            readable: false,
        }
    }

    pub fn with_children(self, children: Vec<ChildEntry>) -> Self {
        Self { children, ..self }
    }
}

#[derive(Debug, Deserialize)]
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
    pub children: Vec<ChildEntry>,
}

impl CourseConfig {
    fn from_raw(raw: RawCourseConfig, children: Vec<ChildEntry>) -> Self {
        let RawCourseConfig {
            title,
            description,
            readable,
        } = raw;
        Self {
            title,
            description,
            readable,
            children,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RawDirConfig {
    pub title: String,
    #[serde(default = "true_fn")]
    pub readable: bool,
}

#[derive(Debug, Serialize)]
pub struct DirConfig {
    pub title: String,
    pub readable: bool,
    pub children: Vec<ChildEntry>,
}

impl DirConfig {
    fn from_raw(raw: RawDirConfig, children: Vec<ChildEntry>) -> Self {
        let RawDirConfig { title, readable } = raw;
        Self {
            title,
            readable,
            children,
        }
    }
}
