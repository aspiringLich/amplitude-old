use std::{any, result};

use afire::{Request, Status};
use anyhow::{anyhow, bail};
use serde::Serialize;

use crate::{error::StatusError, misc::LoginProvider, state::State};

#[derive(Debug, Serialize)]
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
    /// If the user is an admin
    pub admin: bool,
}

#[derive(Debug)]
pub enum SessionPlatform {
    Github(GithubSession),
    Google(GoogleSession),
}

impl From<GithubSession> for SessionPlatform {
    fn from(x: GithubSession) -> Self {
        Self::Github(x)
    }
}

impl From<GoogleSession> for SessionPlatform {
    fn from(x: GoogleSession) -> Self {
        Self::Google(x)
    }
}

impl Serialize for SessionPlatform {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.as_provider().to_string())
    }
}

#[derive(Debug)]
pub struct GoogleSession {
    pub google_id: String,
    pub access_token: String,
}

#[derive(Debug)]
pub struct GithubSession {
    pub github_id: u64,
    pub login: String,
    pub token: String,
}

pub fn get_session(app: &State, req: &Request) -> result::Result<Session, StatusError> {
    let token = req
        .cookies
        .get("session")
        .ok_or(StatusError::from(anyhow!("No session")))?;

    if token == "LOGOUT" {
        return Err(anyhow!("User logged out").into());
    }

    let session = app
        .db
        .session()
        .get_session(token)
        .ok()
        .ok_or(anyhow!("Invalid session"))?;

    Ok(session)
}

pub fn assert_admin(session: &Session) -> result::Result<(), StatusError> {
    if session.admin {
        return Ok(());
    }

    Err(StatusError {
        status: Status::Unauthorized,
        body: Some(serde_json::json!({ "error": "Unauthorized" }).to_string()),
    })
}

impl SessionPlatform {
    pub fn as_provider(&self) -> LoginProvider {
        match self {
            SessionPlatform::Github(_) => LoginProvider::Github,
            SessionPlatform::Google(_) => LoginProvider::Google,
        }
    }
}
