use amplitude_markdown::{parse::read_article, state::article::ArticleConfig};

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
        let s = String::from_utf8_lossy(&req.body);
        let req: ArticleReq =
            serde_json::from_str(&s).context(Status::BadRequest, "Bad Request")?;

        let body = read_article(&state.config, &req.article)
            .context(Status::NotFound, "Article not found")?;

        let config = state
            .parse_state
            .get_article_config(&req.article)
            .context(Status::NotFound, "Article not found")?;
        let response = ArticleResponse { config, body };

        Ok(Response::new()
            .text(serde_json::to_string(&response)?)
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
