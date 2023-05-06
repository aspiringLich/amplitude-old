use super::*;

#[derive(Debug, Deserialize)]
pub struct CourseReq {
    course: String,
}

pub fn attach(_server: &mut Server<State>) {
    // // Returns the html for a course
    // server.handled_stateful_route(Method::POST, "/api/course", |state, req| {
    //     let s = String::from_utf8_lossy(&req.body);
    //     let req: CourseReq = serde_json::from_str(&s)?;
    //     let state = &state.parse_state.write();
    //     let cfg = state
    //         .get_course_config(&req.course)
    //         .context("Invalid course")?;

    //     Ok(Response::new()
    //         .text(serde_json::to_string(cfg)?)
    //         .content(Content::JSON))
    // });
}
