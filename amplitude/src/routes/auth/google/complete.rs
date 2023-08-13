use std::time::Duration;

use afire::{internal::encoding::url, HeaderType, Method, Response, Server, SetCookie, Status};
use serde_json::Value;

use crate::{
    misc::{current_epoch, rand_str, LoginProvider, SESSION_LENGTH},
    session::{GoogleSession, Session, SessionPlatform},
    state::State,
};

pub fn attach(server: &mut Server<State>) {
    server.stateful_route(Method::GET, "/auth/google/complete", move |app, req| {
        // Get Code from URI
        let code = match req.query.get("code") {
            Some(i) => i,
            _ => return Response::new().text("No Auth Code Found"),
        };

        // Get and verify state
        let state = match req.query.get("state") {
            Some(i) => i,
            _ => return Response::new().text("No State Found"),
        };
        let state = match app.db.auth().get_oauth(LoginProvider::Google, state) {
            Ok(i) => i,
            Err(_) => return Response::new().text("Invalid state"),
        };

        if current_epoch() - state >= 60 * 10 {
            return Response::new().text("State Expired");
        }

        // Get Access Token
        let cfg = app.config.auth.google_oauth.as_ref().unwrap();
        let resp = ureq::post("https://oauth2.googleapis.com/token")
            .timeout(Duration::from_secs(app.config.server.req_duration))
            .send_form(&[
                ("grant_type", "authorization_code"),
                ("client_secret", &cfg.client_secret),
                ("client_id", &cfg.client_id),
                ("code", &url::decode(code).unwrap()),
                (
                    "redirect_uri",
                    &format!("{}/auth/google/complete", cfg.external_url),
                ),
            ])
            .unwrap()
            .into_reader();

        // Parse Response and net Token
        let token = serde_json::from_reader::<_, Value>(resp).unwrap();
        let access_token = token
            .get("access_token")
            .and_then(|x| x.as_str())
            .expect("No Access Token!?");

        // Get User Info
        let user_raw = ureq::get("https://www.googleapis.com/oauth2/v1/userinfo")
            .set("Authorization", &format!("Bearer {access_token}"))
            .timeout(Duration::from_secs(app.config.server.req_duration))
            .call()
            .unwrap()
            .into_reader();

        // Parse JSON
        let user = serde_json::from_reader::<_, Value>(user_raw).unwrap();
        let id = user.get("id").and_then(|x| x.as_str()).expect("No ID");
        let name = user.get("name").and_then(|x| x.as_str()).expect("No Name");
        let avatar = user
            .get("picture")
            .and_then(|x| x.as_str())
            .expect("No Picture");

        let google = GoogleSession {
            google_id: id.to_owned(),
            access_token: access_token.to_owned(),
        };
        let token = rand_str(10);
        let session = Session {
            platform: SessionPlatform::Google(google),
            token: token.to_owned(),
            id: rand_str(10),
            name: name.to_owned(),
            avatar: avatar.to_owned(),
            signup: current_epoch(),
        };

        app.db
            .session()
            .add_session(&session, req.headers.get(HeaderType::UserAgent))
            .unwrap();

        let cookie = SetCookie::new("session", token)
            .path("/")
            .max_age(SESSION_LENGTH);

        Response::new()
            .status(Status::TemporaryRedirect)
            .header("Cache-Control", "no-store")
            .header("Location", "/")
            .cookie(cookie)
    });
}
