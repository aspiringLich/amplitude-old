use afire::{Method, Response, Server};
use serde::Deserialize;

use crate::{error::HandledRoute, routes::json, session::get_session, state::State};

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/problem/progress/save", |app, req| {
        let session = get_session(&app, req)?;

        #[derive(Debug, Deserialize)]
        struct ProgressSaveRequest {
            problem_id: u64,
            code: String,
        }

        let data = json::<ProgressSaveRequest>(req)?;
        app.db
            .misc()
            .save_problem_progress(&session.id, data.problem_id, &data.code)?;

        Ok(Response::new())
    });

    server.handled_stateful_route(Method::GET, "/api/problem/progress/load", |app, req| {
        let session = get_session(&app, req)?;

        #[derive(Debug, Deserialize)]
        struct ProgressLoadRequest {
            problem_id: u64,
        }

        let ProgressLoadRequest { problem_id } = json(req)?;
        app.db
            .misc()
            .load_problem_progress(&session.id, problem_id)?;

        todo!()
    });
}
