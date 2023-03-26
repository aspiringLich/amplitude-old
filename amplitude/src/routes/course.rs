use super::*;

pub fn attach(server: &mut Server) {
    // Serves a specific course page
    server.handled_route(Method::GET, "/course/{course}/{article}", |req| {
        let course = req.param("course").unwrap();
        let article = req.param("article").unwrap();

        let path = config::OUTPUT
            .join(course)
            .join(article)
            .with_extension("html");
        let body = std::fs::read_to_string(path).status(Status::NotFound, "Article not found")?;

        Ok(Response::new().text(&body).content(Content::HTML))
    });
}
