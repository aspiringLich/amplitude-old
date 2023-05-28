use std::{
    collections::{HashMap, HashSet},
    default::default,
    fs,
    path::PathBuf,
};

use super::course::{RawCourseConfig, Track};
use crate::items::{article::Article, quiz::Quiz, ItemType};
use amplitude_common::config::Config;
use anyhow::Context;
use comrak::{
    parse_document_refs, Arena, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions,
    ListStyleType, RefMap,
};

#[derive(Debug)]
pub struct MarkdownContext {
    pub options: ComrakOptions,
    pub refs: RefMap,
}

/// Storing information about what weve parsed so far
#[derive(Debug)]
pub struct CourseParseContext {
    pub title: String,
    pub description: String,
    path: PathBuf,
    output_path: PathBuf,
    markdown_context: MarkdownContext,
    items: HashMap<String, ItemType>,
    tracks: HashMap<String, Track>,
}

impl CourseParseContext {
    pub fn new(
        path: PathBuf,
        markdown_context: MarkdownContext,
        config: &Config,
    ) -> anyhow::Result<Self> {
        let course: RawCourseConfig =
            toml::from_str(&fs::read_to_string(path.join("course.toml"))?)?;

        Ok(Self {
            title: course.title,
            description: course.description,
            path,
            output_path: config.parse.output_path.clone().into(),
            markdown_context,
            tracks: default(),
            items: default(),
        })
    }

    pub fn add_track(&mut self, id: String, track: Track) {
        self.tracks.insert(id, track);
    }
}

/// Sort of like `ParseContext` but scoped to something specific
#[derive(Debug)]
pub struct ItemContext<'a> {
    context: &'a mut CourseParseContext,
    id: String,
}

impl<'a> ItemContext<'a> {
    /// Return the id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the `ItemType` of the item
    #[must_use]
    pub fn write_article(&self, html: &str) -> Result<(), std::io::Error> {
        fs::write(
            self.context
                .output_path
                .join(&self.id)
                .with_extension("html"),
            html,
        )
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
    pub fn from(
        context: &'a mut CourseParseContext,
        track: &str,
        id: &str,
    ) -> anyhow::Result<Self> {
        if context.items.contains_key(id) {
            anyhow::bail!("Duplicate item id: {}", id);
        }
        let id = id.to_string();
        context
            .tracks
            .get_mut(track)
            .context("Track does not exist")?
            .items
            .push(id.clone());

        Ok(Self {
            context,
            id: id.to_string(),
        })
    }

    /// Add another scope to the item id, used when passing the context from an
    /// item to some child, e.g. a quiz in an article
    pub fn add_scope(&'a mut self, scope: &str) -> Self {
        let id = self.id.clone() + "/" + scope;

        Self {
            context: self.context,
            id,
        }
    }
}
