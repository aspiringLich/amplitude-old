use super::*;

pub fn attach<T: Send + Sync>(server: &mut Server<T>) {
    // Returns the html for a course
    server.handled_route(Method::POST, "/api/article", |req| {
        #[derive(Deserialize, Debug)]
        struct ArticleRequest {
            course: String,
            article: String,
        }

        let req: ArticleRequest = serde_json::from_str(&String::from_utf8(req.body.clone())?)?;

        let path = config::OUTPUT
            .join(req.course)
            .join(req.article)
            .with_extension("html");
        let body =
            std::fs::read_to_string(path).status(Status::NotFound, "Article not found: {req:?}")?;

        Ok(Response::new().text(body).content(Content::HTML))
    });
}
