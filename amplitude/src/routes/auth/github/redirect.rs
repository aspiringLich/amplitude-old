use afire::{Method, Response, Server};
use rand::Rng;

use crate::{database::Database, misc::LoginProvider, state::State};

pub fn attach(server: &mut Server<State>) {
    if server.app().config.github_oauth.is_none() {
        return;
    }

    server.stateful_route(Method::GET, "/auth/github/redirect", move |app, _| {
        let state = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(10)
            .map(|x| x as char)
            .collect::<String>();

        app.db().add_oauth(LoginProvider::Github, &state).unwrap();

        let cfg = app.config.github_oauth.as_ref().unwrap();
        Response::new().status(308).header(
            "Location",
            format!(
                "https://github.com/login/oauth/authorize?client_id={}&state={}",
                cfg.app_id, state
            ),
        )
    });
}
