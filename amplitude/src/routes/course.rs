use super::*;

pub fn attach<T: Send + Sync>(server: &mut Server<T>) {
    // Serves a specific course page
    server.handled_route(Method::GET, "/api/course/{course}/{article}", |req| {
        let course = req.param("course").unwrap();
        let article = req.param("article").unwrap();

        let path = config::OUTPUT
            .join(course)
            .join(article)
            .with_extension("html");
        let body = std::fs::read_to_string(path).status(Status::NotFound, "Article not found")?;

        Ok(Response::new().text(&body).content(Content::HTML))
    });
    // Serves the index page for a course
    server.handled_route(Method::GET, "/api/course/{course}", |req| {
        let course = req.param("course").unwrap();

        let path = config::OUTPUT.join(course).join("index.html");
        let body = std::fs::read_to_string(path).status(Status::NotFound, "Course not found")?;

        Ok(Response::new().text(&body).content(Content::HTML))
    });
}
