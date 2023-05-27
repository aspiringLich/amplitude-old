use std::{
    collections::{HashMap, HashSet},
    default::default,
};

use crate::items::{
    article::{ArticleConfig, RawArticleConfig},
    quiz::Quiz,
};

use super::track::Track;

/// Storing information about what weve parsed so far
#[derive(Debug, Default)]
pub struct ParseContext {
    item_ids: HashSet<String>,
    tracks: HashMap<String, Track>,
    articles: HashMap<String, ArticleConfig>,
    quizzes: HashMap<String, Quiz>,
}

impl ParseContext {
    pub fn new() -> Self {
        Self { ..default() }
    }

    pub fn add_track(&mut self, track: Track) {
        self.tracks.insert(track.id.clone(), track);
    }
}

/// Sort of like `ParseContext` but scoped to something specific
#[derive(Debug)]
pub struct ItemContext<'a> {
    context: &'a mut ParseContext,
    id: String,
}

impl<'a> ItemContext<'a> {
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
