use amplitude_runner::lang::Language;

use super::*;

use crate::{error::HandledRoute, state::State};

#[derive(Debug, Deserialize)]
struct EvaluateReq {
    code: String,
    lang: Language,
    id: String,
}

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/evaluate", |state, req| {
        let body: EvaluateReq = json(req)?;

        let parse_data = state.parse_data();
        let e = parse_data
            .exercises
            .get(&body.id)
            .with_context(Status::NotFound, || format!("Exercise `{}` not found", body.id))?;

        let id = body.id.split_once('/').unwrap().1;
        let results = e
            .run_tests(&body.lang, &body.code, id, &state.config)
            .context(Status::InternalServerError, "Error running tests")?;

        Ok(Response::new()
            .text(serde_json::to_string(&results)?)
            .content(Content::JSON))
    });
}
