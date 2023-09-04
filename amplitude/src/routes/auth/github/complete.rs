use std::time::Duration;

use afire::{HeaderType, Method, Query, Response, Server, SetCookie, Status};
use anyhow::Context;
use serde::Deserialize;

use crate::{
    error::HandledRoute,
    misc::{current_epoch, rand_str, LoginProvider, SESSION_LENGTH},
    session::{GithubSession, Session, SessionPlatform},
    state::State,
};

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::GET, "/auth/github/complete", move |app, req| {
        // Get Code from URI
        let code = req.query.get("code").context("No auth code found")?;

        // Get and verify state
        let state = req.query.get("state").context("No state found")?;
        let state = app
            .db
            .auth()
            .get_oauth(LoginProvider::Github, state)
            .context("Invalid state")?;

        if current_epoch() - state >= 60 * 10 {
            return Ok(Response::new().text("State Expired"));
        }

        // Get Access Token
        let cfg = app.config.auth.github_oauth.as_ref().unwrap();
        let resp = ureq::post("https://github.com/login/oauth/access_token")
            .query("client_secret", &cfg.app_secret)
            .query("client_id", &cfg.app_id)
            .query("code", code)
            .timeout(Duration::from_secs(app.config.server.req_duration))
            .call()?
            .into_string()?;

        // Parse Response and net Token
        let token = Query::from_body(&resp);
        let token = token.get("access_token").expect("No Access Token!?");

        // Get User Info
        let user_raw = ureq::get("https://api.github.com/user")
            .set("Authorization", &format!("token {token}"))
            .call()?
            .into_reader();

        // Parse Jason
        #[derive(Deserialize)]
        struct GithubUser {
            id: u64,
            login: String,
            name: String,
            avatar_url: String,
        }

        let user = serde_json::from_reader::<_, GithubUser>(user_raw)?;

        let github = GithubSession {
            github_id: user.id,
            login: user.login,
            token: token.to_owned(),
        };
        let token = rand_str(10);
        let session = Session {
            platform: SessionPlatform::Github(github),
            token: token.to_owned(),
            id: rand_str(10),
            name: user.name,
            avatar: user.avatar_url,
            signup: current_epoch(),
            admin: false,
        };

        app.db
            .session()
            .add_session(&session, req.headers.get(HeaderType::UserAgent))?;

        let cookie = SetCookie::new("session", token)
            .path("/")
            .max_age(SESSION_LENGTH);

        Ok(Response::new()
            .status(Status::TemporaryRedirect)
            .header("Cache-Control", "no-store")
            .header("Location", "/")
            .cookie(cookie))
    });
}
