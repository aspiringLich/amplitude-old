use anyhow::ensure;
use anyhow::Context;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::RwLock,
};

pub mod config;
pub mod quiz;

use serde::{Deserialize, Serialize};

use self::config::ArticleConfig;

#[derive(Debug, Default)]
pub struct FileEntry {
    pub name: String,
    pub children: Vec<FileEntry>,
    pub readable: bool,
}

#[derive(Debug, Default)]
pub struct Track {
    pub name: String,
    pub description: String,
    pub files: Vec<FileEntry>,
}

#[derive(Debug, Default)]
pub struct ParseState {
    pub options: comrak::ComrakOptions,
    quizzes: HashMap<(PathBuf, String), quiz::Quiz>,
    articles: HashMap<PathBuf, ArticleConfig>,
    tracks: HashMap<PathBuf, Vec<Track>>,
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

    pub fn get_article_config(&self, article: &Path) -> Option<&ArticleConfig> {
        self.articles.get(article)
    }

    pub fn insert_article_config(
        &mut self,
        article: &Path,
        config: ArticleConfig,
    ) -> Option<ArticleConfig> {
        self.articles.insert(article.to_path_buf(), config)
    }
}
