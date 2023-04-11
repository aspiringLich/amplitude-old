use std::sync::Arc;

use afire::Request;
use anyhow::{anyhow, bail};

use crate::{database::Database, state::State};

pub struct Session {
    /// Platform specific things
    pub platform: SessionPlatform,
    /// Session token
    pub token: String,
    /// Amplify user id
    pub id: String,
    /// User's name
    pub name: String,
    /// URL to their avatar
    pub avatar: String,
    /// The time they signed up (epoch secs)
    pub signup: u64,
}

pub enum SessionPlatform {
    Github(GithubSession),
    Google(GoogleSession),
}

pub struct GoogleSession {
    pub google_id: String,
    pub access_token: String,
}

pub struct GithubSession {
    pub github_id: String,
    pub login: String,
    pub token: String,
}

pub fn get_session(app: Arc<State>, req: &Request) -> anyhow::Result<Session> {
    let token = req.cookies.get("session").ok_or(anyhow!("No session"))?;

    if token == "LOGOUT" {
        bail!("User logged out");
    }

    let session = app
        .db()
        .get_session(token)
        .ok()
        .ok_or(anyhow!("Invalid session"))?;

    Ok(session)
}
