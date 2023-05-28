use std::{collections::HashSet, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::items::{article::Article, parse_item, quiz::Quiz};

use super::{context::CourseParseContext, *};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawCourseConfig {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawTrack {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Debug)]
pub struct Track {
    pub title: String,
    pub description: String,
    pub items: Vec<String>,
}

impl Track {
    pub fn from_raw(raw: RawTrack) -> anyhow::Result<Self> {
        Ok(Self {
            title: raw.title,
            description: raw.description,
            items: Vec::new(),
        })
    }
}

pub fn parse_course(path: PathBuf, config: &Config) -> anyhow::Result<CourseParseContext> {
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
    let arena = Arena::new();

    let refs = {
        let header = fs::read_to_string(path.join("header.md"))?;
        Some(parse_document_refs(&arena, &header))
    }
    .unwrap_or(RefMap::new());

    let md_ctx = MarkdownContext { options, refs };
    let mut context = CourseParseContext::new(path.clone(), md_ctx, config)?;

    for dir in fs::read_dir(&path)? {
        let dir = dir?;
        let path = dir.path();

        if !path.is_dir() {
            continue;
        }

        let file_name = dir.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name.starts_with('.') {
            continue;
        }
        parse_track(path, &mut context)
            .with_context(|| format!("While parsing track {file_name}"))?;
    }

    Ok(context)
}

fn strip_prefix(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap()[3..].to_string()
}

pub fn parse_track(path: PathBuf, context: &mut CourseParseContext) -> anyhow::Result<()> {
    let track: RawTrack = toml::from_str(&fs::read_to_string(path.join("track.toml"))?)
        .context("While parsing `track.toml`")?;
    let track = Track::from_raw(track)?;
    // skip first 3 characters ("00-item" -> "item")
    let track_id = strip_prefix(&path);
    context.add_track(track_id.clone(), track);

    for item in fs::read_dir(&path)? {
        let item = item?;
        let path = item.path();
        if !path.is_dir() {
            continue;
        }
        let file_name = item.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name.starts_with('.') {
            continue;
        }

        let id = strip_prefix(&path);

        parse_item(&path, ItemContext::from(context, &track_id, &id)?)
            .with_context(|| format!("While parsing item at path `{}`", path.to_string_lossy()))?;
    }

    Ok(())
}
