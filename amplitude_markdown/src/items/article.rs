use std::io::{self, BufRead, Read};

use anyhow::Context;
use serde::de::DeserializeOwned;

use crate::parse::parse_md;

use super::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawArticle {
    pub title: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct Article {
    pub title: String,
    pub body: String,
}

impl Article {
    pub fn from_raw(raw: RawArticle, body: String) -> Self {
        Self {
            title: raw.title,
            body,
        }
    }
}

impl Item for Article {
    fn parse_from_dir(
        dir: &Path,
        contents: DirContents,
        context: &mut DataContext,
        _: &Config,
    ) -> anyhow::Result<ItemType> {
        ensure!(contents.contains("article.md"), "article.md");

        let (raw, s): (RawArticle, String) = parse_frontmatter(&dir.join("article.md"))
            .context("While reading article / parsing frontmatter header")?;
        let html = parse_md(&s, context).context("While parsing article markdown")?;

        let article = Article::from_raw(raw, html);
        Ok(ItemType::Article(article))
    }
}

pub fn parse_frontmatter<T: DeserializeOwned>(path: &Path) -> anyhow::Result<(T, String)> {
    let file = fs::File::open(path).unwrap();
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();

    while line.trim().is_empty() {
        reader.read_line(&mut line)?;
    }
    anyhow::ensure!(
        line.trim() == "---",
        "Did not find Frontmatter header on article (Headers start with `---`)"
    );

    line = String::new();
    let mut header = String::new();

    while !matches!(reader.read_line(&mut line), Ok(0)) {
        if line.trim() == "---" {
            let config: T = toml::from_str(&header).context("while parsing frontmatter toml")?;
            let mut rest = vec![];
            reader.read_to_end(&mut rest).unwrap();
            let rest = String::from_utf8(rest).context("Invalid utf-8 in file")?;
            return Ok((config, rest));
        }

        header.push_str(&line);
        line = String::new();
    }

    anyhow::bail!(
        "Did not find end of Frontmatter header on path {}",
        path.display()
    )
}
