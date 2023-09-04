use afire::{Method, Response, Server, Status};
use serde::Deserialize;
use serde_json::json;

use crate::error::{HandledRoute, StatusContext};
use crate::session::get_session;
use crate::State;

use super::json;

#[derive(Debug, Deserialize)]
struct ClassJoinRequest {
    class_id: u64,
}

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::POST, "/api/class/join", |app, req| {
        let session = get_session(&app, req)
            .context(Status::Unauthorized, json!({ "error": "Unauthorized" }))?;

        let body = json::<ClassJoinRequest>(req)?;
        if let Err(e) = app.db.user().join_class(&session.id, body.class_id) {
            return Ok(Response::new()
                .text(json!({ "error": e }))
                .status(Status::BadRequest));
        }

        Ok(Response::new())
    });
}
