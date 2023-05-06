use super::*;


pub fn attach(server: &mut Server<State>) {
    // Serves the json for a quiz
    // server.handled_stateful_route(Method::POST, "/api/quiz", |state, req| {
    //     let s = req.body_str();
    //     let req: QuizReq = serde_json::from_str(&s)?;

    //     let parse_state = state.parse_state.read();
    //     let quiz = parse_state
    //         .get_quiz(&req.article, req.id)
    //         .status(Status::NotFound, format!("Quiz not found: {s:?}"))?;

    //     Ok(Response::new()
    //         .text(serde_json::to_string(&quiz)?)
    //         .content(Content::JSON))
    // });
}
