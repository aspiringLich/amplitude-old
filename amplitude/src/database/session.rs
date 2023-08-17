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
        let id = match &session.platform {
            SessionPlatform::Github(p) => this.query_row(
                include_str!("./sql/auth/github/upsert_login.sql"),
                params![
                    session.id,
                    p.github_id,
                    session.name,
                    p.login,
                    session.avatar,
                    p.token
                ],
                |x| x.get::<_, String>(0),
            ),
            SessionPlatform::Google(p) => this.query_row(
                include_str!("./sql/auth/google/upsert_login.sql"),
                params![
                    session.id,
                    p.google_id,
                    session.name,
                    session.avatar,
                    p.access_token,
                ],
                |x| x.get::<_, String>(0),
            ),
        }
        .unwrap_or_else(|_| session.id.to_owned());

        this.execute(
            include_str!("./sql/insert_sessions.sql"),
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

        Ok(match platform.into() {
            LoginProvider::Github => {
                this.query_row("SELECT * FROM github_users WHERE id = ?1", [user_id], |x| {
                    Ok(Session {
                        id: x.get::<_, String>(0)?,
                        name: x.get::<_, String>(2)?,
                        avatar: x.get::<_, String>(4)?,
                        signup: x.get::<_, u64>(6)?,
                        token: token.to_string(),
                        platform: SessionPlatform::Github(GithubSession {
                            github_id: x.get::<_, u64>(1)?,
                            login: x.get::<_, String>(3)?,
                            token: x.get::<_, String>(5)?,
                        }),
                    })
                })?
            }
            LoginProvider::Google => {
                this.query_row("SELECT * FROM google_users WHERE id = ?1", [user_id], |x| {
                    Ok(Session {
                        id: x.get::<_, String>(0)?,
                        name: x.get::<_, String>(2)?,
                        avatar: x.get::<_, String>(3)?,
                        signup: x.get::<_, u64>(5)?,
                        token: token.to_string(),
                        platform: SessionPlatform::Google(GoogleSession {
                            google_id: x.get::<_, String>(1)?,
                            access_token: x.get::<_, String>(4)?,
                        }),
                    })
                })?
            }
        })
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
