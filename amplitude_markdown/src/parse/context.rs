use super::{course::{Track, CategoryConfig}, parse_md, RawParseData};
use amplitude_runner::exercise::Exercise;
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
    context: &'a mut RawParseData,
    id: String,
}

impl<'a> DataContext<'a> {
    pub fn next_seed(&mut self) -> u64 {
        let seed = self.context.seed;
        self.context.seed += 1;
        seed
    }

    /// Add an exercise to the context
    pub fn add(&mut self, exercise: Exercise) -> anyhow::Result<()> {
        debug!(
            "{:24} (id: {})",
            "Adding exercise to context",
            &self.id
        );
        self.context.exercises.insert(self.id.clone(), exercise);
        let (cat, id) = self.id.split_once('/').expect("ID has slash");
        self.context
            .tree
            .entry(cat.to_string())
            .or_default()
            .push(id.to_string());
        // if !track_id.is_empty() {
        //     let id = self.id.clone();
        //     self.get_course_tracks()?
        //         .iter_mut()
        //         .rfind(|track| track.id == track_id)
        //         .with_context(|| format!("Track `{track_id}` not found"))?
        //         .items
        //         .push(id);
        // }

        Ok(())
    }

    pub fn add_category(&mut self, category: CategoryConfig) {
        debug!(
            "{:24} (id: {})",
            "Adding category to context",
            &self.id
        );
        self.context.categories.insert(self.id.clone(), category);
        self.context.tree.insert(self.id.clone(), vec![]);
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
    
    /// Return the `MarkdownContext` used for parsing markdown mutably
    pub fn markdown_context_mut(&mut self) -> &mut MarkdownContext {
        &mut self.context.markdown_context
    }
    
    /// Return the `ComrakOptions` used for parsing markdown
    pub fn markdown_options(&self) -> &ComrakOptions {
        &self.context.markdown_context.options
    }

    /// Create a new `ItemContext` from a `ParseContext` and an id
    pub fn new(context: &'a mut RawParseData, id: &str) -> anyhow::Result<Self> {
        if context.categories.contains_key(id) {
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
