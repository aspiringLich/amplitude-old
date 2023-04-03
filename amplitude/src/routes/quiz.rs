use serde::Deserialize;

use super::*;

#[derive(Deserialize, Debug, Clone)]
struct QuizRequest {
    course: String,
    article: String,
    id: String,
}

pub fn attach(server: &mut Server<State>) {
    // Serves the json for a quiz
    server.handled_stateful_route(Method::POST, "/api/quiz", |state, req| {
        let req: QuizRequest = serde_json::from_str(&String::from_utf8(req.body.clone())?)?;
        let cpy = req.clone();

        let parse_state = state.parse.lock().unwrap();
        let quiz = parse_state
            .questions
            .get(&(req.course, req.article, req.id))
            .status(Status::NotFound, format!("Quiz not found {:?}", cpy))?;
        Ok(Response::new()
            .text(serde_json::to_string(&quiz)?)
            .content(Content::JSON))
    });
}
