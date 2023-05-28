use std::{collections::HashMap, default::default, fs, path::PathBuf};

use super::{course::Track, parse_md, RawCourseData};
use crate::items::ItemType;
use amplitude_common::config::Config;
use anyhow::Context;
use comrak::{ComrakOptions, RefMap};
use tracing::debug;

#[derive(Debug)]
pub struct MarkdownContext {
    pub options: ComrakOptions,
    pub refs: RefMap,
}

/// Sort of like `ParseContext` but scoped to something specific
#[derive(Debug)]
pub struct DataContext<'a> {
    context: &'a mut RawCourseData,
    id: String,
}

impl<'a> DataContext<'a> {
    /// Add an item to the context
    #[must_use]
    pub fn add_item(&mut self, item: ItemType, track_id: &str) -> anyhow::Result<()> {
        debug!(
            "{:24} ({:8} id: {})",
            "Adding item to context",
            item.to_string(),
            &self.id
        );
        self.context.items.insert(self.id.clone(), item);
        if !track_id.is_empty() {
            self.context
                .tracks
                .get_mut(track_id)
                .with_context(|| format!("Track `{track_id}` not found"))?
                .items
                .push(self.id.clone());
        }

        Ok(())
    }

    pub fn add_track(&mut self, track: Track, track_id: String) {
        debug!("{:24} (id: {})", "Adding track to context", track_id);
        self.context.tracks.insert(track_id, track);
    }

    /// Return the id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the `MarkdownContext` used for parsing markdown
    pub fn markdown_context(&self) -> &MarkdownContext {
        &self.context.markdown_context
    }

    /// Return the `ComrakOptions` used for parsing markdown
    pub fn markdown_options(&self) -> &ComrakOptions {
        &self.context.markdown_context.options
    }

    /// Create a new `ItemContext` from a `ParseContext` and an item id
    pub fn new(context: &'a mut RawCourseData, id: &str) -> anyhow::Result<Self> {
        if context.items.contains_key(id) {
            anyhow::bail!("Duplicate item id: {}", id);
        }
        let id = id.to_string();

        Ok(Self {
            context,
            id: id.to_string(),
        })
    }

    /// Scope this `ItemContext` to a something else
    pub fn scope<F, T>(&mut self, scope: &str, f: F) -> T
    where
        F: FnOnce(&mut DataContext) -> T,
    {
        let id = self.id.clone();
        self.id = id.clone() + "/" + scope;
        let out = f(self);
        self.id = id;

        return out;
    }

    #[must_use]
    pub fn parse_md(&mut self, p: &mut impl ParseMarkdown) -> anyhow::Result<()> {
        p.parse_md(self)?;
        Ok(())
    }
}

pub trait ParseMarkdown {
    fn parse_md(&mut self, ctx: &mut DataContext) -> anyhow::Result<()>;
}

impl ParseMarkdown for String {
    fn parse_md(&mut self, ctx: &mut DataContext) -> anyhow::Result<()> {
        *self = parse_md(self, ctx)?;
        Ok(())
    }
}
