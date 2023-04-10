use crate::config::INPUT;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleConfig {
    pub title: String,
}

pub fn parse_frontmatter(article: &Path) -> anyhow::Result<ArticleConfig> {
    let path = INPUT.join(article);
    let input = fs::read_to_string(path).context("While reading article")?;
    ensure!(
        input.starts_with("---"),
        "Article frontmatter header must start with `---`"
    );
    let header = input
        .split('\n')
        .skip(1)
        .take_while(|line| line != &"---")
        .fold(String::new(), |str, line| str + "\n" + line);

    toml::from_str(&header).context("While parsing frontmatter")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub name: String,
    pub path: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tracks {
    pub tracks: Vec<Track>,
}
