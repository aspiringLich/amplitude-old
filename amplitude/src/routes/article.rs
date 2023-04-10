use super::*;

pub fn attach<T: Send + Sync>(server: &mut Server<T>) {
    // Returns the html for a course
    server.handled_route(Method::POST, "/api/article", |req| {
        let s = String::from_utf8(req.body.clone())?;
        let req: ArticleReq = serde_json::from_str(&s)?;

        let body = std::fs::read_to_string(req.path()).status(
            Status::NotFound,
            format!("Article not found: {:?}", req.display()),
        )?;

        Ok(Response::new().text(body).content(Content::HTML))
    });
}
