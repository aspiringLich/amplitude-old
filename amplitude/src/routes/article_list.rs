use super::*;

#[derive(Deserialize)]
struct CourseReq {
    course: String,
}

/// Returns the list of articles in a course
pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/article-list", |state, req| {
        let s = String::from_utf8_lossy(&req.body);
        let req: CourseReq = serde_json::from_str(&s).context(Status::BadRequest, "Bad Request")?;
        let state = &state.parse_state;

        let course = state
            .courses
            .get(&req.course)
            .context(Status::NotFound, "Course not found")?;

        Ok(Response::new()
            .text(serde_json::to_string(course)?)
            .content(Content::JSON))
    });
}
