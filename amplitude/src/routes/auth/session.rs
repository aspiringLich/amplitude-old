//! Gets user information for the current session.
//! Includes user id, name, avatar, and signup date.

use afire::{Content, Method, Response, Server};
use anyhow::Context;
use serde_json::json;

use crate::{error::HandledRoute, misc::OkResponse, session::get_session, state::State};

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::GET, "/auth/session", move |app, req| {
        let session = get_session(&app, req).context(json!({ "error": "Unauthorized" }))?;

        Response::new()
            .text(serde_json::to_string(&session)?)
            .content(Content::JSON)
            .ok()
    });
}
