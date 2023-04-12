use afire::{Content, Method, Response, Server};
use serde::Serialize;
use serde_json::json;

use crate::{error::HandledRoute, misc::OkResponse, session::get_session, state::State};

#[derive(Serialize)]
struct SessionResponse {
    github: bool,
    google: bool,
}

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::GET, "/auth/session", move |app, req| {
        let session = match get_session(app, req) {
            Ok(session) => session,
            Err(_) => {
                return Response::new()
                    .status(401)
                    .text(json!({ "error": "Unauthorized" }))
                    .content(Content::JSON)
                    .ok();
            }
        };

        Response::new()
            .text(serde_json::to_string(&session)?)
            .content(Content::JSON)
            .ok()
    });
}
