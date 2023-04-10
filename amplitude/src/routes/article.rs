use amplitude_common::state::config::ArticleConfig;

use super::*;

#[derive(Serialize, Debug)]
struct ArticleResponse<'a> {
    body: String,
    config: &'a ArticleConfig,
}

pub fn attach(server: &mut Server<App>) {
    // Returns the html for a course
    server.handled_stateful_route(Method::POST, "/api/article", |state, req| {
        let s = String::from_utf8(req.body.clone())?;
        let req: ArticleReq = serde_json::from_str(&s)?;

        let body = fs::read_to_string(req.file_path()).status(
            Status::NotFound,
            format!("Article not found: {:?}", req.display()),
        )?;

        let parse_state = &state.documents.read();
        let config = parse_state
            .get_article_config(&req.path())
            .context("Article config not found")?;
        let response = ArticleResponse { config, body };

        Ok(Response::new()
            .text(serde_json::to_string(&response)?)
            .content(Content::JSON))
    });
}
