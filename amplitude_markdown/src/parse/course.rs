use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing_subscriber::registry::Data;

use crate::items::parse_item;

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CourseConfig {
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
    pub id: String,
    pub title: String,
    pub description: String,
    pub items: Vec<String>,
}

impl Track {
    pub fn from_raw(raw: RawTrack, id: String) -> anyhow::Result<Self> {
        Ok(Self {
            id,
            title: raw.title,
            description: raw.description,
            items: Vec::new(),
        })
    }
}

pub fn parse_course(
    path: PathBuf,
    config: &Config,
    data: &mut RawCourseData,
) -> anyhow::Result<()> {
    let arena = Arena::new();
    let refs = {
        let header = fs::read_to_string(path.join("header.md"))?;
        Some(parse_document_refs(&arena, &header))
    }
    .unwrap_or(RefMap::new());
    data.markdown_context.refs = refs;

    let course_id = path.file_name().to_string();
    for dir in fs::read_dir(&path)? {
        let dir = dir?;
        let path = dir.path();

        if !path.is_dir() {
            continue;
        }

        let file_name = dir.file_name();
        let track_id = file_name.to_str().unwrap();
        if track_id.starts_with('.') {
            continue;
        }

        let mut ctx = DataContext::new(data, &course_id)?;

        parse_track(path, &mut ctx).with_context(|| format!("While parsing track {track_id}"))?;
    }

    Ok(())
}

fn strip_prefix(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap()[3..].to_string()
}

pub fn parse_track(path: PathBuf, ctx: &mut DataContext) -> anyhow::Result<()> {
    let track: RawTrack = toml::from_str(&fs::read_to_string(path.join("track.toml"))?)
        .context("While parsing `track.toml`")?;
    let track_id = strip_prefix(&path);
    let track = Track::from_raw(track, track_id.clone())?;

    ctx.add_track(track, track_id.clone());

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

        ctx.scope(&id, |ctx| {
            parse_item(&path, ctx, &track_id)
                .with_context(|| format!("While parsing item at path `{}`", path.to_string_lossy()))
        })?
    }

    Ok(())
}
