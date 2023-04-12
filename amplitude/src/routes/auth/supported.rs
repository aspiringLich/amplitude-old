//! Returns the supported authentication methods.
//! (Google and/or GitHub)

use afire::{Content, Method, Response, Server};
use serde::Serialize;

use crate::state::State;

#[derive(Serialize)]
struct SupportedResponse {
    github: bool,
    google: bool,
}

pub fn attach(server: &mut Server<State>) {
    server.stateful_route(Method::GET, "/auth/supported", move |app, _req| {
        let supported = SupportedResponse {
            github: app.config.github_oauth.is_some(),
            google: app.config.google_oauth.is_some(),
        };

        Response::new()
            .text(serde_json::to_string(&supported).unwrap())
            .content(Content::JSON)
    });
}
