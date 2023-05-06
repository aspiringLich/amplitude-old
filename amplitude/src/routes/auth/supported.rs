//! Returns the supported authentication methods.
//! (Google and/or GitHub)

use afire::{Content, Method, Response, Server};
use serde::Serialize;
use serde_json::json;

use crate::state::State;

#[derive(Serialize)]
struct SupportedOauth {
    name: String,
    path: String,
}

pub fn attach(server: &mut Server<State>) {
    server.stateful_route(Method::GET, "/auth/supported", move |app, _req| {
        let mut supported = Vec::new();

        if app.config.auth.github_oauth.is_some() {
            supported.push(SupportedOauth {
                name: "GitHub".to_string(),
                path: "/auth/github/redirect".to_string(),
            });
        }

        if app.config.auth.google_oauth.is_some() {
            supported.push(SupportedOauth {
                name: "Google".to_string(),
                path: "/auth/google/redirect".to_string(),
            });
        }

        Response::new()
            .text(json!(supported))
            .content(Content::JSON)
    });
}
