use amplitude_common::state::config::{parse_frontmatter, ArticleConfig};

use super::*;

#[derive(Serialize, Debug)]
struct ArticleResponse {
    body: String,
    config: ArticleConfig,
}

pub fn attach(server: &mut Server<State>) {
    // Returns the html for a course
    server.handled_route(Method::POST, "/api/article", |req| {
        let s = String::from_utf8(req.body.clone())?;
        let req: ArticleReq = serde_json::from_str(&s)?;

        let body = fs::read_to_string(req.path()).status(
            Status::NotFound,
            format!("Article not found: {:?}", req.display()),
        )?;

        let response = ArticleResponse {
            config: parse_frontmatter(&**req)?,
            body,
        };

        Ok(Response::new()
            .text(serde_json::to_string(&response)?)
            .content(Content::JSON))
    });
}
