use std::io::{self, BufRead, Read};

use serde::de::DeserializeOwned;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleConfig {
    pub title: String,
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

impl ParseState {
    pub fn get_article_config(&self, article: &Path) -> Option<&article::ArticleConfig> {
        self.article_config.get(article)
    }

    pub fn insert_article_config(
        &mut self,
        article: &Path,
        config: article::ArticleConfig,
    ) -> Option<article::ArticleConfig> {
        self.article_config.insert(article.to_path_buf(), config)
    }
}
