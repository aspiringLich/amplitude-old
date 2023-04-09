use anyhow::ensure;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleConfig {
    pub title: String,
    pub author: String,
}

pub fn parse_frontmatter(input: &str) -> anyhow::Result<ArticleConfig> {
    ensure!(
        input.starts_with("---"),
        "Article frontmatter header must start with `---`"
    );
    let header = input
        .split('\n')
        .take_while(|line| line != &"---")
        .fold(String::new(), |str, line| str + line);
    dbg!(&header);

    toml::from_str(&header).context("While parsing frontmatter")
}
