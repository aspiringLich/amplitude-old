
use serde::{Deserialize, Serialize};


// #[derive(Deserialize, Debug, Clone)]
// #[serde(deny_unknown_fields)]
// pub struct RawCourseConfig {
//     pub title: String,
//     pub description: String,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoryConfig {
    pub title: String,
    pub description: String,
    // pub icon: PathBuf,
    // pub index: String,
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

// impl CourseConfig {
//     /// Sorta messy but whatever.
//     ///
//     /// From the information in the config file (`RawCourseConfig`), generates
//     /// the full `CourseConfig` struct, which includes the path to the icon file
//     /// (and maybe other stuff in the future).
//     pub fn from_raw(
//         raw: RawCourseConfig,
//         path: &Path,
//         cfg: &Config,
//         data: &mut RawCourseData,
//         course_id: &str,
//     ) -> anyhow::Result<Self> {
//         let icon = fs::read_dir(path)?
//             .find(|x| {
//                 x.as_ref()
//                     .is_ok_and(|x| x.file_name().to_string_lossy().starts_with("icon."))
//             })
//             .with_context(|| format!("Could not find icon file in `{}`", path.display()))?
//             .unwrap();
//         let icon = icon.path();
//         anyhow::ensure!(cfg
//             .parse
//             .image_extensions
//             .contains(&icon.extension().unwrap().to_string()));

//         let new_path = register_file(&icon, cfg).context("While registering icon file")?;

//         let index = parse_md(
//             &fs::read_to_string(path.join("index.md"))?,
//             &mut DataContext::new(data, &course_id)?,
//         )
//         .context("While parsing index.md")?;

//         Ok(Self {
//             title: raw.title,
//             description: raw.description,
//             icon: new_path,
//             index,
//         })
//     }
// }

// pub fn parse_course(path: PathBuf, data: &mut RawCourseData, cfg: &Config) -> anyhow::Result<()> {
//     let arena = Arena::new();
//     let refs = {
//         let header = fs::read_to_string(path.join("header.md"))?;
//         Some(parse_document_refs(&arena, &header))
//     }
//     .unwrap_or(RefMap::new());
//     data.markdown_context.refs = refs;

//     let course_id = path.file_name().to_string();

//     // insert course info
//     let course: CategoryConfig = toml::from_str(&fs::read_to_string(path.join("course.toml"))?)?;

//     data.categories.insert(course_id.clone(), course);
//     data.tracks.insert(course_id.clone(), Vec::new());

//     for dir in fs::read_dir(&path)? {
//         let dir = dir?;
//         let path = dir.path();

//         if !path.is_dir() {
//             continue;
//         }

//         let mut ctx = DataContext::new(data, &course_id)?;
//         let file_name = dir.file_name();

//         if file_name == "exercises" {
//             for item in fs::read_dir(&path)
//                 .with_context(|| format!("While reading dir {}", path.display()))?
//             {
//                 let item = item?;
//                 let path = item.path();

//                 let file_name = path.file_name().unwrap();
//                 let file_name = file_name.to_string_lossy();

//                 if path.is_dir() {
//                     ctx.scope(&file_name, |ctx| -> anyhow::Result<()> {
//                         let exercise: Exercise =
//                             from_directory(&path, ctx, cfg).context("While parsing exercise")?;
//                         ctx.add(ItemType::Exercise(exercise), "")
//                             .context("While adding item to course")?;
//                         Ok(())
//                     })?;
//                 }
//             }
//             continue;
//         }

//         let track_id = file_name.to_str().unwrap();
//         if track_id.starts_with('.') {
//             continue;
//         }

//         parse_track(path, &mut ctx, cfg)
//             .with_context(|| format!("While parsing track {track_id}"))?;
//     }

//     Ok(())
// }

// fn strip_prefix(path: &Path) -> String {
//     path.file_name().unwrap().to_str().unwrap()[3..].to_string()
// }

// pub fn parse_track(path: PathBuf, ctx: &mut DataContext, cfg: &Config) -> anyhow::Result<()> {
//     let track: RawTrack = toml::from_str(&fs::read_to_string(path.join("track.toml"))?)
//         .context("While parsing `track.toml`")?;
//     let track_id = strip_prefix(&path);
//     let track = Track::from_raw(track, track_id.clone()).context("While parsing `RawTrack`")?;

//     ctx.add_track(track)?;

//     for item in fs::read_dir(&path)? {
//         let item = item?;
//         let path = item.path();
//         if path.is_dir() {
//             continue;
//         }

//         let file_name = item.file_name();
//         let file_name = file_name.to_str().unwrap();
//         if file_name.starts_with('.') {
//             continue;
//         }
//         if file_name == "track.toml" {
//             continue;
//         }

//         let id = strip_prefix(&path)
//             .split_once('.')
//             .map(|x| x.0.to_string())
//             .unwrap_or_else(|| strip_prefix(&path));

//         ctx.scope(&id, |ctx| {
//             parse_item(&path, ctx, &track_id, cfg)
//                 .with_context(|| format!("While parsing item at path `{}`", path.to_string_lossy()))
//         })?
//     }

//     Ok(())
// }
