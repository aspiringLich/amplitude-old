use super::*;

#[derive(Debug, Deserialize)]
pub struct TrackReq {
    course: String,
    track: String,
}

pub fn attach(server: &mut Server<State>) {
    // // Returns the html for a course
    // server.handled_stateful_route(Method::POST, "/api/article_list", |state, req| {
    //     let s = String::from_utf8_lossy(&req.body);
    //     let req: TrackReq = serde_json::from_str(&s)?;
    //     let state = &state.parse_state.write();
    //     let course = state
    //         .get_course_config(&req.course)
    //         .context("Course not found!")?;
    //     let track = course
    //         .children
    //         .get(&req.track)
    //         .context("Track not found!")?;

    //     Ok(Response::new()
    //         .text(serde_json::to_string(&track)?)
    //         .content(Content::JSON))
    // });
}
