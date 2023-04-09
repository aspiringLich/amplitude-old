use super::*;

pub fn attach<T: Send + Sync>(server: &mut Server<T>) {
    // Returns the html for a course
    server.handled_route(Method::POST, "/api/article", |req| {
        let s = String::from_utf8(req.body.clone())?;
        let req: ArticleReq = serde_json::from_str(&s)?;
        let path = req.into_path();

        let body = std::fs::read_to_string(&path).status(
            Status::NotFound,
            format!("Article not found: {:?}", path.display()),
        )?;

        Ok(Response::new().text(body).content(Content::HTML))
    });
}
