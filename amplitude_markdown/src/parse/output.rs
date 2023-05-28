use super::{*, context::CourseParseContext};

/// The output from parsing the articles.
#[derive(Debug)]
pub struct ParseData {}

impl ParseData {
    /// Create a new `ParseData` from the given `courses`.
    pub fn from_courses(
        courses: HashMap<String, CourseParseContext>,
        config: &Config,
    ) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}
