use afire::{Method, Response, Server, Status};
use serde::Deserialize;
use serde_json::json;

use crate::error::HandledRoute;
use crate::session::{assert_admin, get_session};
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
    // < { "class_name": "..." }
    server.handled_stateful_route(Method::POST, "/api/class/join", |app, req| {
        let session = get_session(&app, req)?;

        let body = json::<ClassRequest>(req)?;
        if let Err(e) = app.db.user().join_class(&session.id, body.class_id) {
            return Ok(Response::new()
                .text(json!({ "error": e }))
                .status(Status::BadRequest));
        }

        let name = app.db.misc().class_name(body.class_id)?;
        Ok(Response::new().text(json!({ "class_name": name })))
    });

    // Removes the current user from a class
    // > POST /api/class/leave
    // > Cookie: <session>
    // > { "class_id": 123 }
    server.handled_stateful_route(Method::POST, "/api/class/leave", |app, req| {
        let session = get_session(&app, req)?;

        let body = json::<ClassRequest>(req)?;
        if let Err(e) = app.db.user().leave_class(&session.id, body.class_id) {
            return Ok(Response::new()
                .text(json!({ "error": e }))
                .status(Status::BadRequest));
        }

        Ok(Response::new())
    });

    // Lists all classes the current user is in.
    // > GET /api/class/list
    // > Cookie: <session>
    // < { "classes": [ { "class_id": 123, "class_name": "CS 101", "date_joined": 1234567890 } ] }
    server.handled_stateful_route(Method::GET, "/api/class/list", |app, req| {
        let session = get_session(&app, req)?;
        let classes = app.db.user().list_classes(&session.id)?;

        Ok(Response::new().text(json!(classes)))
    });

    // Allows an admin to create a class.
    // > POST /api/class/create
    // > Cookie: <session>
    // > { "name": "CS Principles Period 2" }
    // < { "id": 3428 }
    server.handled_stateful_route(Method::POST, "/api/class/create", |app, req| {
        let session = get_session(&app, req)?;
        assert_admin(&session)?;

        #[derive(Debug, Deserialize)]
        struct CreateClassRequest {
            name: String,
        }

        let CreateClassRequest { name } = json(req)?;
        let id = app.db.misc().unique_class_id()?;

        app.db.misc().create_class(id, &name)?;

        Ok(Response::new().text(json!({ "id": id })))
    });

    // Allows an admin to kick a user from a class.
    // > POST /api/class/kick
    // > Cookie: <session>
    // > { "class_id": 123, "user_id": "abc" }
    // < {}
    server.handled_stateful_route(Method::POST, "/api/class/kick", |app, req| {
        let session = get_session(&app, req)?;
        assert_admin(&session)?;

        #[derive(Debug, Deserialize)]
        struct ClassKickRequest {
            class_id: u64,
            user_id: String,
        }

        let ClassKickRequest { class_id, user_id } = json(req)?;

        if let Err(e) = app.db.user().leave_class(&user_id, class_id) {
            return Ok(Response::new()
                .text(json!({ "error": e }))
                .status(Status::BadRequest));
        }

        Ok(Response::new().text(json!({})))
    });
}
