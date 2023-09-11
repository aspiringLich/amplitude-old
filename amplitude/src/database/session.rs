use crate::{
    misc::{current_epoch, LoginProvider, SESSION_LENGTH},
    session::{GithubSession, GoogleSession, Session, SessionPlatform},
};

use super::{Db, SessionMeta};
use anyhow::Result;
use derive_more::Deref;
use rusqlite::params;

#[derive(Deref)]
pub struct SessionDb<'a>(pub(super) &'a Db);

impl<'a> SessionDb<'a> {
    pub fn add_session(&self, session: &Session, agent: Option<&str>) -> anyhow::Result<()> {
        let this = self.lock();

        // Add platform specific login data
        let id: String = match &session.platform {
            SessionPlatform::Github(p) => {
                this.execute(
                    include_str!("./sql/auth/github/upsert_login.sql"),
                    params![session.id, p.github_id, p.login, p.token,],
                )?;
                this.query_row(
                    "SELECT id FROM github_users WHERE github_id = ?",
                    [p.github_id],
                    |row| row.get(0),
                )?
            }
            SessionPlatform::Google(p) => {
                this.execute(
                    include_str!("./sql/auth/google/upsert_login.sql"),
                    params![session.id, p.google_id, p.access_token,],
                )?;
                this.query_row(
                    "SELECT id FROM google_users WHERE google_id = ?",
                    [&p.google_id],
                    |row| row.get(0),
                )?
            }
        };

        // Add generic login data
        this.execute(
            include_str!("./sql/auth/upsert_login.sql"),
            params![id, session.name, session.avatar],
        )?;

        // Add session to database
        this.execute(
            include_str!("./sql/session/insert_sessions.sql"),
            params![
                id,
                session.token,
                session.platform.as_provider() as u8,
                agent
            ],
        )?;

        Ok(())
    }

    pub fn get_session(&self, token: &str) -> anyhow::Result<Session> {
        let this = self.lock();
        let (created, user_id, platform) = this.query_row(
            "SELECT created, user_id, platform FROM sessions WHERE session_id = ?",
            [token],
            |x| {
                Ok((
                    x.get::<_, u64>(0)?,
                    x.get::<_, String>(1)?,
                    x.get::<_, u8>(2)?,
                ))
            },
        )?;

        // Expire session after 30 days
        if current_epoch() - created > SESSION_LENGTH {
            self.delete_session(token)?;
            return Err(anyhow::anyhow!("Session expired"));
        }

        let platform_data: SessionPlatform = match platform.into() {
            LoginProvider::Github => this.query_row(
                "SELECT github_id, login, token FROM github_users WHERE id = ?1",
                [&user_id],
                |x| {
                    Ok(GithubSession {
                        github_id: x.get(0)?,
                        login: x.get(1)?,
                        token: x.get(2)?,
                    }
                    .into())
                },
            )?,
            LoginProvider::Google => this.query_row(
                "SELECT google_id, access_token FROM google_users WHERE id = ?1",
                [&user_id],
                |x| {
                    Ok(GoogleSession {
                        google_id: x.get(0)?,
                        access_token: x.get(1)?,
                    }
                    .into())
                },
            )?,
        };

        Ok(this.query_row(
            "SELECT name, avatar_url, created, admin FROM users WHERE id = ?",
            [&user_id],
            |x| {
                Ok(Session {
                    platform: platform_data,
                    token: token.to_owned(),
                    id: user_id.to_owned(),
                    name: x.get(0)?,
                    avatar: x.get(1)?,
                    signup: x.get(2)?,
                    admin: x.get(3)?,
                })
            },
        )?)
    }

    pub fn delete_session(&self, token: &str) -> anyhow::Result<()> {
        self.lock()
            .execute("DELETE FROM sessions WHERE session_id = ?1", [token])?;
        Ok(())
    }

    pub fn delete_sessions(&self, session: &Session) -> anyhow::Result<()> {
        self.lock()
            .execute("DELETE FROM sessions WHERE user_id = ?1", [&session.id])?;
        Ok(())
    }

    pub fn get_sessions(&self, session: &Session) -> anyhow::Result<Vec<SessionMeta>> {
        let this = self.lock();
        let mut stmt = this
            .prepare("SELECT session_id, created, user_agent FROM sessions WHERE user_id = ?1")?;

        let sessions = stmt
            .query_map([&session.id], |x| {
                Ok((
                    x.get::<_, String>(0)?,
                    x.get::<_, u64>(1)?,
                    x.get::<_, Option<String>>(2)?,
                ))
            })?
            .map(Result::unwrap)
            .collect::<Vec<_>>();

        Ok(sessions)
    }
}
