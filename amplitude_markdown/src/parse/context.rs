use super::{course::Track, parse_md, RawCourseData};
use crate::items::ItemType;
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
    pub fn next_seed(&mut self) -> u64 {
        let seed = self.context.seed;
        self.context.seed += 1;
        seed
    }

    /// Add an item to the context
    pub fn add_item(&mut self, item: ItemType, track_id: &str) -> anyhow::Result<()> {
        debug!(
            "{:24} ({:8} id: {})",
            "Adding item to context",
            item.to_string(),
            &self.id
        );
        self.context.items.insert(self.id.clone(), item);
        if !track_id.is_empty() {
            let id = self.id.clone();
            self.get_course_tracks()?
                .iter_mut()
                .rfind(|track| track.id == track_id)
                .with_context(|| format!("Track `{track_id}` not found"))?
                .items
                .push(id);
        }

        Ok(())
    }

    fn get_course_tracks(&mut self) -> anyhow::Result<&mut Vec<Track>> {
        let course_id = self.id.split_once('/').map(|(a, _)| a).unwrap_or(&self.id);
        let tracks = self
            .context
            .tracks
            .get_mut(course_id)
            .with_context(|| format!("Course `{course_id}` not found"))?;

        Ok(tracks)
    }

    pub fn add_track(&mut self, track: Track) -> anyhow::Result<()> {
        debug!("{:24} (id: {})", "Adding track to context", track.id);
        self.get_course_tracks()?.push(track);
        Ok(())
    }

    /// Return the id
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return the course id
    pub fn course_id(&self) -> &str {
        self.id.split_once('/').unwrap_or_default().0
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

        Ok(Self { context, id })
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

        out
    }

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
