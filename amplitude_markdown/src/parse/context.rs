use std::{
    collections::{HashMap, HashSet},
    default::default,
};

use comrak::{ComrakOptions, RefMap};

use crate::items::{
    article::{ArticleConfig, RawArticleConfig},
    quiz::Quiz,
};

use super::track::Track;

#[derive(Debug)]
pub struct MarkdownContext<'a> {
    pub options: &'a ComrakOptions,
    pub refs: &'a RefMap,
}

/// Storing information about what weve parsed so far
#[derive(Debug)]
pub struct ParseContext<'a> {
    markdown_context: &'a MarkdownContext<'a>,
    item_ids: HashSet<String>,
    tracks: HashMap<String, Track>,
    articles: HashMap<String, ArticleConfig>,
    quizzes: HashMap<String, Quiz>,
}

impl<'a> ParseContext<'a> {
    pub fn new(markdown_context: &'a MarkdownContext) -> Self {
        Self {
            markdown_context,
            item_ids: default(),
            tracks: default(),
            articles: default(),
            quizzes: default(),
        }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.insert(track.id.clone(), track);
    }
}

/// Sort of like `ParseContext` but scoped to something specific
#[derive(Debug)]
pub struct ItemContext<'a> {
    context: &'a mut ParseContext<'a>,
    id: String,
}

impl<'a> ItemContext<'a> {
    /// Return the `MarkdownContext` used for parsing markdown
    pub fn markdown_context(&self) -> &'a MarkdownContext<'a> {
        self.context.markdown_context
    }

    /// Return the `ComrakOptions` used for parsing markdown
    pub fn markdown_options(&self) -> &ComrakOptions {
        self.context.markdown_context.options
    }
    
    /// Insert an article config
    pub fn insert_article(&mut self, article: ArticleConfig) {
        self.context.articles.insert(article.id.clone(), article);
    }
    
    /// Insert a quiz
    pub fn insert_quiz(&mut self, quiz: Quiz) {
        self.context.quizzes.insert(quiz.id.clone(), quiz);
    }
    
    /// Create a new `ItemContext` from a `ParseContext` and an item id
    pub fn from(context: &'a mut ParseContext, id: &str) -> anyhow::Result<Self> {
        if context.item_ids.contains(id) {
            anyhow::bail!("Duplicate item id: {}", id);
        }

        Ok(Self {
            context,
            id: id.to_string(),
        })
    }

    /// Add another scope to the item id, used when passing the context from an
    /// item to some child, e.g. a quiz in an article
    pub fn add_scope(&self, scope: &str) -> Self {
        let id = self.id + "/" + scope;
        self.context.item_ids.insert(id.clone());

        Self {
            context: self.context,
            id,
        }
    }
}
