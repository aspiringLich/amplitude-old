use afire::{Method, Response, Server, Status};
use serde::Deserialize;
use serde_json::json;

use crate::error::{HandledRoute, StatusContext};
use crate::session::get_session;
use crate::State;

use super::json;

#[derive(Debug, Deserialize)]
struct ClassRequest {
    class_id: u64,
}

pub fn attach(server: &mut Server<State>) {
    // Adds the current user to a class.
    // Will error if the user is already in the class or if the class does not exist (crate::database::user::ClassJoinError).
    // > POST /api/class/join
    // > Cookie: <session>
    // > { "class_id": 123 }
    server.handled_stateful_route(Method::POST, "/api/class/join", |app, req| {
        let session = get_session(&app, req)
            .context(Status::Unauthorized, json!({ "error": "Unauthorized" }))?;

        let body = json::<ClassRequest>(req)?;
        if let Err(e) = app.db.user().join_class(&session.id, body.class_id) {
            return Ok(Response::new()
                .text(json!({ "error": e }))
                .status(Status::BadRequest));
        }

        Ok(Response::new())
    });

    // Removes the current user from a class
    // > POST /api/class/leave
    // > Cookie: <session>
    // > { "class_id": 123 }
    server.handled_stateful_route(Method::POST, "/api/class/leave", |app, req| {
        let session = get_session(&app, req)
            .context(Status::Unauthorized, json!({ "error": "Unauthorized" }))?;

        let body = json::<ClassRequest>(req)?;
        if let Err(e) = app.db.user().leave_class(&session.id, body.class_id) {
            return Ok(Response::new()
                .text(json!({ "error": e }))
                .status(Status::BadRequest));
        }

        Ok(Response::new())
    });

    server.handled_stateful_route(Method::GET, "/api/class/list", |app, req| {
        let session = get_session(&app, req)
            .context(Status::Unauthorized, json!({ "error": "Unauthorized" }))?;
        let classes = app.db.user().list_classes(&session.id)?;

        Ok(Response::new().text(json!(classes)))
    });
}
