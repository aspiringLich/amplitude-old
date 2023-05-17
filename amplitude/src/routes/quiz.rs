use super::*;

#[derive(Debug, Deserialize)]
struct QuizReq {
    article: String,
    id: String,
}

pub fn attach(server: &mut Server<State>) {
    // Serves the json for a quiz
    server.handled_stateful_route(Method::POST, "/api/quiz", |state, req| {
        let s = req.body_str();
        let req: QuizReq = serde_json::from_str(&s)?;

        let quiz = state
            .parse_state
            .get_quiz(&req.article, &req.id)
            .with_context(Status::NotFound, || {
                format!("Quiz not found! ({}/{})", &req.article, &req.id)
            })?;

        Ok(Response::new()
            .text(serde_json::to_string(&quiz)?)
            .content(Content::JSON))
    });
}
