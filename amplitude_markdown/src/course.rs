use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CourseItem {
    Article(String),
    Course(Vec<CourseItem>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(into = "CourseItem")]
pub struct Course {
    pub index: String,
    pub articles: Vec<CourseItem>,
}

impl From<Course> for CourseItem {
    fn from(mut value: Course) -> Self {
        value.articles.push(CourseItem::Article(value.index));
        CourseItem::Course(value.articles)
    }
}