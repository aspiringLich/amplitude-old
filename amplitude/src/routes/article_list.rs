use super::*;

#[derive(Deserialize, Debug)]
struct CourseReq {
    course: String,
}

/// Returns the list of articles in a course
pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/article-list", |state, req| {
        let req: CourseReq = json(req)?;
        let state = &state.parse_data;

        let course = state
            .courses
            .get(&req.course)
            .context(Status::NotFound, "Course not found")?;

        Ok(Response::new()
            .text(serde_json::to_string(course)?)
            .content(Content::JSON))
    });
    server.handled_stateful_route(Method::GET, "/api/course-list", |state, _req| {
        let state = &state.parse_data;

        Ok(Response::new()
            .text(serde_json::to_string(&state.courses)?)
            .header("Access-Control-Allow-Origin", "*")
            .content(Content::JSON))
    });
}
