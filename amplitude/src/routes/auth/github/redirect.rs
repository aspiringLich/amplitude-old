use afire::{Method, Response, Server, Status};

use crate::{
    database::Database,
    misc::{rand_str, LoginProvider},
    state::State,
};

pub fn attach(server: &mut Server<State>) {
    if server.app().config.github_oauth.is_none() {
        return;
    }

    server.stateful_route(Method::GET, "/auth/github/redirect", move |app, _| {
        let state = rand_str(10);

        app.db().add_oauth(LoginProvider::Github, &state).unwrap();

        let cfg = app.config.github_oauth.as_ref().unwrap();
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
