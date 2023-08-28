pub mod context;
pub mod course;
pub mod inject;
pub mod link_concat;

use crate::{
    items::article::parse_frontmatter,
    path::{DirectoryContent, FromDirectory},
    OsStrToString,
};
use amplitude_common::{
    config::{Config, ParseConfig},
    default,
};
use amplitude_runner::exercise::Exercise;
use anyhow::Context;
use comrak::{
    nodes::AstNode, parse_document_refs, Arena, ComrakExtensionOptions, ComrakOptions,
    ComrakRenderOptions, ListStyleType, RefMap,
};
use git2::build::RepoBuilder;
use link_concat::link_concat_callback;
use serde::{ser::SerializeMap, Serialize, Serializer};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    fs::{self, File},
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    vec,
};
use tracing::{info, warn};

use self::{
    context::{DataContext, MarkdownContext},
    course::{CategoryConfig, Track},
    inject::InjectData,
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
    if config.args.pull {
        info!(
            "Deleting `{}` and recloning repo...",
            config.parse.clone_path
        );
        clone_repo(&config.parse).context("While cloning repo")?;
    } else {
        info!(
            "Using local files in `{}` for articles",
            config.parse.clone_path
        );
    }

    // delete the generated asset files
    if Path::new(&config.parse.asset_path).exists() {
        fs::remove_dir_all(&config.parse.asset_path).context("While clearing asset path")?;
    }
    fs::create_dir(&config.parse.asset_path).context("While creating asset path")?;

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

    let mut data = RawParseData::new(md_ctx);
    for item in fs::read_dir(&config.parse.clone_path)? {
        let item = item?;
        let path = item.path();
        if !path.is_dir() {
            continue;
        }
        let name = path.file_name().to_string();
        if name.starts_with('.') {
            continue;
        }

        let scope = || -> anyhow::Result<()> {
            let mut ctx = DataContext::new(&mut data, &name).context("While creating context")?;
            let header =
            File::open(&path.join("header.md")).context("While openning header file")?;

            let (category, s): (CategoryConfig, _) =
                parse_frontmatter(&header).context("While parsing frontmatter for header")?;
            ctx.add_category(category);

            let arena = Arena::new();
            let refs = parse_document_refs(&arena, &s);
            
            let original = ctx.markdown_context().refs.clone();
            ctx.markdown_context_mut().refs.extend(refs);

            for item in
                fs::read_dir(path).with_context(|| format!("While reading category {}", name))?
            {
                let item = item?;
                let path = item.path();
                if !path.is_dir() {
                    continue;
                }
                let name = path.file_name().to_string();

                ctx.scope(&name, |ctx| -> anyhow::Result<()> {
                    let exercise = Exercise::from_directory(
                        &DirectoryContent::new(&path).context("While getting directory content")?,
                        ctx,
                        &config,
                    ).context("While getting `Exercise`")?;
                    
                    ctx.add(exercise).context("While adding exercise")?;
                    
                    Ok(())
                })
                .context("While parsing exercise")?;
            }

            data.markdown_context.refs = original;

            Ok(())
        };
        scope().with_context(|| format!("While parsing category `{}`", name))?;
    }

    // dbg!(&data);

    Ok(ParseData::from_raw(data).context("While converting `RawParseData` to `ParseData`")?)
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
        arena,
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
/// Has full access to `ItemContext`,
/// Will also return the `InjectData` for the item
pub(crate) fn parse_md_full(
    input: &str,
    ctx: &mut DataContext,
) -> anyhow::Result<(String, InjectData)> {
    // do things
    let arena = Arena::new();
    let node = parse_into_ast(input, ctx.markdown_context(), ctx.id(), &arena)?;

    let mut data = default();
    inject::inject(node, ctx, &mut data)?;
    parse_ast(node, ctx.markdown_context()).map(|s| (s, data))
}

/// Parse the input `md` and return the output `html`.
/// Has full access to `ItemContext`
pub(crate) fn parse_md(input: &str, ctx: &mut DataContext) -> anyhow::Result<String> {
    // do things
    let arena = Arena::new();
    let node = parse_into_ast(input, ctx.markdown_context(), ctx.id(), &arena)?;

    let mut data = default();
    inject::inject(node, ctx, &mut data)?;
    parse_ast(node, ctx.markdown_context())
}

pub(crate) fn parse_ast<'a>(
    node: &'a AstNode<'a>,
    ctx: &MarkdownContext,
) -> anyhow::Result<String> {
    let mut cm = vec![];
    comrak::format_html(node, &ctx.options, &mut cm).context("while parsing AST to html")?;
    String::from_utf8(cm).context("While converting html to string")
}

/// Storing information about what weve parsed so far
#[derive(Debug)]
pub struct RawParseData {
    pub categories: HashMap<String, CategoryConfig>,
    // items: HashMap<String, ItemType>,
    // tracks: HashMap<String, Vec<Track>>,
    exercises: HashMap<String, Exercise>,
    tree: HashMap<String, Vec<String>>,
    markdown_context: MarkdownContext,
    pub seed: u64,
}

impl RawParseData {
    pub fn new(md_ctx: MarkdownContext) -> Self {
        Self {
            markdown_context: md_ctx,
            categories: default(),
            exercises: default(),
            tree: default(),
            seed: default(),
        }
    }
}

/// Storing information about what weve parsed so far
#[derive(Debug, Default)]
pub struct ParseData {
    pub categories: HashMap<String, CategoryConfig>,
    // pub items: HashMap<String, ItemType>,
    // pub tracks: HashMap<String, Vec<Track>>,
    pub exercises: HashMap<String, Exercise>,
    pub tree: HashMap<String, Vec<String>>,
}

impl ParseData {
    pub fn from_raw(data: RawParseData) -> anyhow::Result<Self> {
        Ok(Self {
            categories: data.categories,
            exercises: data.exercises,
            tree: data.tree,
        })
    }

    // impl RawCourseData {
    //     pub fn new(markdown_context: MarkdownContext) -> anyhow::Result<Self> {
    //         // let course: RawCourseConfig =
    //         //     toml::from_str(&fs::read_to_string(path.join("course.toml"))?)?;
    //         // let id = path
    //         //     .file_name()
    //         //     .context("Course path is not a directory")?
    //         //     .to_string_lossy()
    //         //     .to_string();

    //         Ok(Self {
    //             // id,
    //             // title: course.title,
    //             // description: course.description,
    //             // output_path: config.parse.output_path.clone().into(),
    //             categories: default(),
    //             markdown_context,
    //             tracks: default(),
    //             items: default(),
    //             seed: default(),
    //         })
    //     }

    //     pub fn add_track(&mut self, course_id: String, track: Track) -> anyhow::Result<()> {
    //         self.tracks
    //             .get_mut(&course_id)
    //             .with_context(|| format!("Course `{course_id}` not found"))?
    //             .push(track);
    //         Ok(())
    //     }
}

/// Takes a path to a file and throws it in the asset folder to be served in
/// wherever we want to serve it.
///
/// Returns the new path of the file (from the servers perspective)
pub fn register_file<P: AsRef<Path>>(path: P, cfg: &Config) -> anyhow::Result<PathBuf> {
    let path = path.as_ref();

    // hash the path to generate (hopefully) unique filename
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);

    let orig_filename = path
        .file_name()
        .context("Expected file to have a name")?
        .to_string_lossy();
    let filename = format!("{:x}-{}", hasher.finish(), orig_filename);

    // copy the file to the asset path
    let new_path = Path::new(&cfg.parse.asset_path).join(&filename);
    assert!(
        !new_path.exists(),
        "Either there was a hash collision or you forgot to clear {}",
        cfg.parse.asset_path
    );
    fs::copy(path, &new_path).context("While copying file")?;

    let path_out = Path::new(&cfg.parse.asset_prefix).join(&filename);
    Ok(path_out)
}
