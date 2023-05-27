use std::collections::HashSet;

use crate::items::{article::ArticleConfig, quiz::Quiz};

use super::*;

pub enum Item {
    Article(ArticleConfig),
    Quiz(Quiz),
}

pub struct CourseContext {
    pub item_ids: HashSet<String>,
}