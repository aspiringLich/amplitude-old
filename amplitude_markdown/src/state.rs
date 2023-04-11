use anyhow::ensure;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub mod article;
pub mod course;
pub mod quiz;

#[derive(Debug, Default)]
pub struct FileEntry {
    pub name: String,
    pub children: Vec<FileEntry>,
    pub readable: bool,
}

#[derive(Debug, Default)]
pub struct ParseState {
    pub options: comrak::ComrakOptions,
    quizzes: HashMap<(PathBuf, String), quiz::Quiz>,
    articles: HashMap<PathBuf, article::ArticleConfig>,
    course_config: HashMap<PathBuf, course::CourseConfig>,
}

impl ParseState {
    pub fn get_quiz(&self, article: &Path, id: String) -> Option<&quiz::Quiz> {
        self.quizzes.get(&(article.to_path_buf(), id))
    }

    pub fn insert_quiz(
        &mut self,
        article: &Path,
        id: &str,
        quiz: quiz::Quiz,
    ) -> Option<quiz::Quiz> {
        self.quizzes
            .insert((article.to_path_buf(), id.to_owned()), quiz)
    }

    pub fn get_article_config(&self, article: &Path) -> Option<&article::ArticleConfig> {
        self.articles.get(article)
    }

    pub fn insert_article_config(
        &mut self,
        article: &Path,
        config: article::ArticleConfig,
    ) -> Option<article::ArticleConfig> {
        self.articles.insert(article.to_path_buf(), config)
    }

    pub fn insert_course(
        &mut self,
        path: &Path,
        config: course::CourseConfig,
    ) -> Option<course::CourseConfig> {
        self.course_config.insert(path.to_path_buf(), config)
    }

    pub fn get_track(&self, path: &Path) -> Option<&course::CourseConfig> {
        self.course_config.get(path)
    }
}
