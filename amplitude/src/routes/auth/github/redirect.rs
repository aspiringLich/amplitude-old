use afire::{Method, Response, Server, Status};

use crate::{
    misc::{rand_str, LoginProvider},
    state::State,
};

pub fn attach(server: &mut Server<State>) {
    server.stateful_route(Method::GET, "/auth/github/redirect", move |app, req| {
        let redirect = req.query.get("r");
        let state = rand_str(10);

        app.db
            .auth()
            .add_oauth(LoginProvider::Github, &state, redirect)
            .unwrap();

        let cfg = app.config.auth.github_oauth.as_ref().unwrap();
        Response::new()
            .status(Status::TemporaryRedirect)
            .header("Cache-Control", "no-store")
            .header(
                "Location",
                format!(
                    "https://github.com/login/oauth/authorize?client_id={}&state={}",
                    cfg.app_id, state
                ),
            )
    });
}
