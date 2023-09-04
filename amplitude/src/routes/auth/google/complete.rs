use std::time::Duration;

use afire::{internal::encoding::url, HeaderType, Method, Response, Server, SetCookie, Status};
use anyhow::Context;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    error::HandledRoute,
    misc::{current_epoch, rand_str, LoginProvider, SESSION_LENGTH},
    session::{GoogleSession, Session, SessionPlatform},
    state::State,
};

pub fn attach(server: &mut Server<State>) {
    server.handled_stateful_route(Method::GET, "/auth/google/complete", move |app, req| {
        // Get Code from URI
        let code = req.query.get("code").context("No auth code found")?;

        // Get and verify state
        let state = req.query.get("state").context("No state found")?;
        let state = app
            .db
            .auth()
            .get_oauth(LoginProvider::Google, state)
            .context("Invalid state")?;

        if current_epoch() - state >= 60 * 10 {
            return Ok(Response::new()
                .status(Status::BadRequest)
                .text("State Expired"));
        }

        // Get Access Token
        let cfg = app.config.auth.google_oauth.as_ref().unwrap();
        let resp = ureq::post("https://oauth2.googleapis.com/token")
            .timeout(Duration::from_secs(app.config.server.req_duration))
            .send_form(&[
                ("grant_type", "authorization_code"),
                ("client_secret", &cfg.client_secret),
                ("client_id", &cfg.client_id),
                (
                    "code",
                    &url::decode(code).context("Invalid URL encoding on")?,
                ),
                (
                    "redirect_uri",
                    &format!("{}/auth/google/complete", cfg.external_url),
                ),
            ])?
            .into_reader();

        // Parse Response and net Token
        let token = serde_json::from_reader::<_, Value>(resp)?;
        let access_token = token
            .get("access_token")
            .and_then(|x| x.as_str())
            .context("No Access Token!?")?;

        // Get User Info
        let user_raw = ureq::get("https://www.googleapis.com/oauth2/v1/userinfo")
            .set("Authorization", &format!("Bearer {access_token}"))
            .timeout(Duration::from_secs(app.config.server.req_duration))
            .call()?
            .into_reader();

        // Parse JSON
        #[derive(Deserialize)]
        struct GoogleUser {
            id: String,
            name: String,
            picture: String,
        }

        let user = serde_json::from_reader::<_, GoogleUser>(user_raw)?;

        let google = GoogleSession {
            google_id: user.id,
            access_token: access_token.to_owned(),
        };
        let token = rand_str(10);
        let session = Session {
            platform: SessionPlatform::Google(google),
            token: token.to_owned(),
            id: rand_str(10),
            name: user.name,
            avatar: user.picture,
            signup: current_epoch(),
            admin: false
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
