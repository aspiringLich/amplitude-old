pub mod context;
pub mod course;
mod inject;
mod link_concat;

use crate::{items::ItemType, parse::course::parse_course, OsStrToString};
use amplitude_common::config::{Config, ParseConfig};
use anyhow::Context;
use comrak::{
    nodes::AstNode, parse_document_refs, Arena, ComrakExtensionOptions, ComrakOptions,
    ComrakRenderOptions, ListStyleType, RefMap,
};
use git2::build::RepoBuilder;
use link_concat::link_concat_callback;
use serde::{ser::SerializeMap, Serialize, Serializer};
use std::{collections::HashMap, default::default, fs, path::Path, vec};
use tracing::{info, warn};

use self::{
    context::{DataContext, MarkdownContext},
    course::{CourseConfig, Track},
};

/// Clones the articles repo
pub fn clone_repo(config: &ParseConfig) -> anyhow::Result<()> {
    let clone_path = &config.clone_path;
    fs::create_dir_all(clone_path)?;

    // delete everything
    for item in fs::read_dir(clone_path)? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    RepoBuilder::new().clone(&config.git_url, Path::new(clone_path))?;

    Ok(())
}

/// Reparses the things and does the things
pub fn parse(config: &Config) -> anyhow::Result<ParseData> {
    if !config.args.local {
        info!("Deleting `{}` and recloning repo... (If you dont want this behavior, run with `--local`)", config.parse.clone_path);
        clone_repo(&config.parse).context("While cloning repo")?;
    } else {
        info!(
            "Using local files in `{}` for articles",
            config.parse.clone_path
        );
    }

    let options = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            tagfilter: true,
            table: true,
            autolink: true,
            tasklist: true,
            superscript: true,
            header_ids: None,
            footnotes: true,
            description_lists: true,
            front_matter_delimiter: Some("---".to_string()),
        },
        parse: default(),
        render: ComrakRenderOptions {
            github_pre_lang: false,
            full_info_string: true,
            unsafe_: true,
            hardbreaks: false,
            width: 0,
            escape: false,
            list_style: ListStyleType::default(),
            sourcepos: false,
        },
    };
    let md_ctx = MarkdownContext {
        options,
        refs: RefMap::new(),
    };

    info!("Parsing articles...");

    let mut data = RawCourseData::new(md_ctx).context("While creating `RawCourseData`")?;
    for item in fs::read_dir(&config.parse.clone_path)? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            let name = path.file_name().to_string();
            if name.starts_with('.') {
                continue;
            }

            parse_course(path, &mut data)
                .with_context(|| format!("While parsing course `{name}`"))?;
        }
    }
    let data = ParseData::from_raw(data).context("While generating `ParseData`")?;

    dbg!(&data);

    Ok(data)
}

fn parse_into_ast<'a>(
    input: &'a str,
    ctx: &MarkdownContext,
    id: &str,
    arena: &'a Arena<AstNode<'a>>,
) -> anyhow::Result<&'a AstNode<'a>> {
    // get the refs
    let mut this_refs = parse_document_refs(&Arena::new(), input);
    this_refs.extend(ctx.refs.clone());

    let ast = comrak::parse_document_with_broken_link_callback(
        &arena,
        input,
        &ctx.options,
        Some(&mut |link| {
            let out = link_concat_callback(link, &this_refs);
            if out.is_none() {
                warn!("Broken link `{link}` in {id}");
            }
            out
        }),
    );
    Ok(ast)
}

/// Parse the input `md` and return the output `html`.
/// Has full access to `ItemContext`
pub(crate) fn parse_md(input: &str, ctx: &mut DataContext) -> anyhow::Result<String> {
    // do things
    let arena = Arena::new();
    let node = parse_into_ast(input, ctx.markdown_context(), ctx.id(), &arena)?;
    inject::inject(&node, ctx)?;
    parse_ast(node, ctx.markdown_context())
}

pub(crate) fn parse_ast<'a>(
    node: &'a AstNode<'a>,
    ctx: &MarkdownContext,
) -> anyhow::Result<String> {
    let mut cm = vec![];
    comrak::format_html(&node, &ctx.options, &mut cm).context("while parsing AST to html")?;
    Ok(String::from_utf8(cm).context("While converting html to string")?)
}

/// Storing information about what weve parsed so far
#[derive(Debug)]
pub struct RawCourseData {
    pub course_data: HashMap<String, CourseConfig>,
    markdown_context: MarkdownContext,
    items: HashMap<String, ItemType>,
    tracks: HashMap<String, Vec<Track>>,
}

/// Storing information about what weve parsed so far
#[derive(Debug)]
pub struct ParseData {
    pub course_data: HashMap<String, CourseConfig>,
    pub items: HashMap<String, ItemType>,
    pub tracks: HashMap<String, Vec<Track>>,
    pub tree: HashMap<String, TreeItem>,
}

fn as_hashmap<K: Serialize, V: Serialize, S: Serializer>(
    list: &[(K, V)],
    s: S,
) -> Result<S::Ok, S::Error> {
    let mut map = s.serialize_map(Some(list.len()))?;
    for (k, v) in list {
        map.serialize_entry(&k, &v)?;
    }
    map.end()
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TreeItem {
    #[serde(serialize_with = "as_hashmap")]
    Course(Vec<(String, TreeItem)>),
    Track(Vec<String>),
}

impl ParseData {
    pub fn from_raw(data: RawCourseData) -> anyhow::Result<Self> {
        let mut courses: HashMap<_, _> = HashMap::new();
        for (id, tracks) in &data.tracks {
            courses.insert(
                id.clone(),
                TreeItem::Course(
                    tracks
                        .iter()
                        .map(|t| (t.id.clone(), TreeItem::Track(t.items.clone())))
                        .collect(),
                ),
            );
        }

        Ok(Self {
            course_data: data.course_data,
            items: data.items,
            tracks: data.tracks,
            tree: courses,
        })
    }
}

impl RawCourseData {
    pub fn new(markdown_context: MarkdownContext) -> anyhow::Result<Self> {
        // let course: RawCourseConfig =
        //     toml::from_str(&fs::read_to_string(path.join("course.toml"))?)?;
        // let id = path
        //     .file_name()
        //     .context("Course path is not a directory")?
        //     .to_string_lossy()
        //     .to_string();

        Ok(Self {
            // id,
            // title: course.title,
            // description: course.description,
            // output_path: config.parse.output_path.clone().into(),
            course_data: default(),
            markdown_context,
            tracks: default(),
            items: default(),
        })
    }

    #[must_use]
    pub fn add_track(&mut self, course_id: String, track: Track) -> anyhow::Result<()> {
        self.tracks
            .get_mut(&course_id)
            .with_context(|| format!("Course `{course_id}` not found"))?
            .push(track);
        Ok(())
    }
}
