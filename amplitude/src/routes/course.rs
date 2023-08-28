use super::*;

#[derive(Deserialize, Debug)]
struct CourseReq {
    course: String,
}

/// Returns the configuration / information needed to display a course page
pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "api/course", |state, req| {
        let mut req: CourseReq = json(req)?;
        req.course = req
            .course
            .strip_prefix('/')
            .unwrap_or(&req.course)
            .to_string();

        let parse_data = state.parse_data();
        let course = parse_data
            .categories
            .get(&req.course)
            .with_context(Status::NotFound, || {
                format!("Course `{}` not found", req.course)
            })?;
        Ok(Response::new()
            .text(serde_json::to_string(&course)?)
            .content(Content::JSON))
    })
}
