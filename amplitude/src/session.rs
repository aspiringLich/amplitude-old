use std::sync::Arc;

use afire::Request;
use anyhow::{anyhow, bail};
use serde::Serialize;

use crate::{misc::LoginProvider, state::State};

#[derive(Serialize)]
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

impl Serialize for SessionPlatform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.as_provider().to_string())
    }
}

pub struct GoogleSession {
    pub google_id: String,
    pub access_token: String,
}

pub struct GithubSession {
    pub github_id: u64,
    pub login: String,
    pub token: String,
}

pub fn get_session(app: &State, req: &Request) -> anyhow::Result<Session> {
    let token = req.cookies.get("session").ok_or(anyhow!("No session"))?;

    if token == "LOGOUT" {
        bail!("User logged out");
    }

    let session = app
        .db
        .session()
        .get_session(token)
        .ok()
        .ok_or(anyhow!("Invalid session"))?;

    Ok(session)
}

impl SessionPlatform {
    pub fn as_provider(&self) -> LoginProvider {
        match self {
            SessionPlatform::Github(_) => LoginProvider::Github,
            SessionPlatform::Google(_) => LoginProvider::Google,
        }
    }
}
