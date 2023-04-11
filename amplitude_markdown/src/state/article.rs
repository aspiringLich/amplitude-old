use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleConfig {
    pub title: String,
}

pub fn parse_article_config(input: &str) -> anyhow::Result<ArticleConfig> {
    ensure!(
        input.starts_with("---"),
        "Did not find Frontmatter header on article (Headers start with `---`)"
    );
    let header = input
        .lines()
        .skip(1)
        .take_while(|line| line != &"---")
        .fold(String::new(), |str, line| str + "\n" + line);

    toml::from_str(&header).context("While parsing frontmatter")
}
