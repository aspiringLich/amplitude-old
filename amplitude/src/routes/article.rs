use amplitude_markdown::state::article::ArticleConfig;
use tracing::trace;

use super::*;

#[derive(Serialize, Debug)]
struct ArticleResponse<'a> {
    body: String,
    config: &'a ArticleConfig,
}

#[derive(Serialize, Debug)]
struct ArticleConfigResponse<'a> {
    config: &'a ArticleConfig,
}

#[derive(Deserialize, Debug)]
struct ArticleReq {
    article: String,
}

pub fn attach(server: &mut Server<State>) {
    // Returns the html for an article
    server.handled_stateful_route(Method::POST, "/api/article", |state, req| {
        let req: ArticleReq = json(&req)?;

        let body = state
            .parse_state
            .read_article(&req.article)
            .with_context(Status::NotFound, || {
                trace!("/api/article: Article {} not found", req.article);
                "Article not found"
            })?;

        let config = state
            .parse_state
            .get_article_config(&req.article)
            .context(Status::NotFound, "Article config not found")?;
        let response = ArticleResponse { config, body };

        Ok(Response::new()
            .text(serde_json::to_string(&response)?)
            .header("Access-Control-Allow-Origin", "*")
            .content(Content::JSON))
    });

    // Returns the config for an article
    server.handled_stateful_route(Method::POST, "/api/article-config", |state, req| {
        let s = String::from_utf8_lossy(&req.body);
        let req: ArticleReq =
            serde_json::from_str(&s).context(Status::BadRequest, "Bad Request")?;

        let config = state
            .parse_state
            .get_article_config(&req.article)
            .context(Status::NotFound, "Article not found")?;
        let response = ArticleConfigResponse { config };

        Ok(Response::new()
            .text(serde_json::to_string(&response)?)
            .content(Content::JSON))
    });
}
