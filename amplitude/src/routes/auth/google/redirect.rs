use afire::{internal::encoding::url, Method, Response, Server, Status};

use rand::Rng;

use crate::{database::Database, misc::LoginProvider, state::State};

pub fn attach(server: &mut Server<State>) {
    server.stateful_route(Method::GET, "/auth/google/redirect", move |app, _| {
        let state = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(10)
            .map(|x| x as char)
            .collect::<String>();

        app.db().add_oauth(LoginProvider::Google, &state).unwrap();

        let cfg = app.config.google_oauth.as_ref().unwrap();
        let redirect = format!(
            "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}/auth/google/complete&response_type=code&scope=profile&state={}",
            cfg.client_id,
            url::encode(&cfg.external_url),
            state
        );

        Response::new()
            .status(Status::TemporaryRedirect)
            .header("Location", &redirect)
            .header("Cache-Control", "no-store")
            .text(format!("Redirecting to {redirect}"))
    });
}
