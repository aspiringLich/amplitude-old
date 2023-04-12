use super::{article::parse_article_config, *};
use std::fs::{self, File};

const fn true_fn() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct DirConfigRaw {
    pub title: String,
    #[serde(default = "true_fn")]
    pub readable: bool,
    pub description: Option<String>,
}

#[derive(Debug, Default)]
pub struct Entry {
    pub name: String,
    pub path: String,
    pub children: Vec<Entry>,
    pub readable: bool,
}

impl Entry {
    pub fn new(path: &Path, state: &mut ParseState) -> anyhow::Result<Self> {
        let mut children = vec![];
        let readable;
        let name;

        if path.is_dir() {
            let mut has_md = false;
            for entry in fs::read_dir(path)? {
                let path = entry?.path();
                if path.is_dir() {
                    let entry = Self::new(&path, state)
                        .with_context(|| format!("while parsing path {}", path.display()))?;
                    if entry.children.len() > 0 || entry.readable {
                        children.push(entry);
                    }
                } else if path.ends_with(".md") {
                    has_md = true;
                    children.push(
                        Self::new(&path, state)
                            .with_context(|| format!("while parsing path {}", path.display()))?,
                    );
                }
            }

            if has_md {
                let index = path.join("index.md");
                anyhow::ensure!(index.exists(), "Expected index.md in directory");

                parse_dir_config(&index)
                    .with_context(|| format!("While parsing {}", index.display()));
            }
        } else if path.is_file() {
            name = state
                .get_article_config(&path)
                .context("Expected article config")?
                .title
                .clone();
        } else {
            anyhow::bail!("Could not access entry")
        }

        Ok(Self {
            name,
            path: path.to_str().unwrap().to_string(),
            children,
            readable,
        })
    }
}

#[derive(Debug, Default)]
pub struct Track {
    pub name: String,
    pub description: String,
    pub files: Vec<Entry>,
}

#[derive(Debug, Default)]
pub struct CourseConfig {
    pub title: String,
    pub tracks: Vec<Track>,
}

