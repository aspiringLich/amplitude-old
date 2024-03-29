//! Views other sessions for the current user.
//! Contains creation date as epoch (secs), user agent, and session id.

use afire::{Content, Method, Response, Server};
use serde::Serialize;
use serde_json::json;

use crate::{error::HandledRoute, misc::OkResponse, session::get_session, state::State};

#[derive(Serialize)]
struct SessionEntry {
    created: u64,
    user_agent: Option<String>,
    id: String,
}

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::GET, "/auth/sessions", move |app, req| {
        let session = match get_session(&app, req) {
            Ok(session) => session,
            Err(_) => {
                return Response::new()
                    .status(401)
                    .text(json!({ "error": "Unauthorized" }))
                    .content(Content::JSON)
                    .ok();
            }
        };

        let sessions = app
            .db
            .session()
            .get_sessions(&session)?
            .into_iter()
            .map(|x| SessionEntry {
                created: x.1,
                user_agent: x.2,
                id: x.0,
            })
            .collect::<Vec<_>>();

        Response::new()
            .text(serde_json::to_string(&sessions)?)
            .content(Content::JSON)
            .ok()
    });
}
