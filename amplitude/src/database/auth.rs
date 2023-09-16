use crate::misc::LoginProvider;

use super::Db;
use anyhow::Result;
use derive_more::Deref;
use rusqlite::params;

#[derive(Deref)]
pub struct AuthDb<'a>(pub(super) &'a Db);

pub struct OAuthState {
    pub created: u64,
    pub redirect: Option<String>,
}

impl<'a> AuthDb<'a> {
    pub fn add_oauth(
        &self,
        service: LoginProvider,
        state: &str,
        redirect: Option<&str>,
    ) -> Result<()> {
        match service {
            LoginProvider::Github => self.lock().execute(
                "INSERT INTO github_oauth_state (state, redirect, created) VALUES (?1, ?2, strftime('%s', 'now'))",
                params![state, redirect],
            ),
            LoginProvider::Google => self.lock().execute(
                "INSERT INTO google_oauth_state (state, redirect, created) VALUES (?1, ?2, strftime('%s', 'now'))",
                params![state, redirect],
            ),
        }?;

        Ok(())
    }

    /// Gets and removes the oauth state
    pub fn get_oauth(&self, service: LoginProvider, state: &str) -> Result<OAuthState> {
        let res = match service {
            LoginProvider::Github => {
                let date = self.lock().query_row(
                    "SELECT created, redirect FROM github_oauth_state WHERE state = ?1",
                    [state],
                    |x| Ok((x.get::<_, u64>(0)?, x.get::<_, Option<String>>(1)?)),
                )?;
                self.lock()
                    .execute("DELETE FROM github_oauth_state WHERE state = ?1", [state])?;
                date
            }
            LoginProvider::Google => {
                let date = self.lock().query_row(
                    "SELECT created, redirect FROM google_oauth_state WHERE state = ?1",
                    [state],
                    |x| Ok((x.get::<_, u64>(0)?, x.get::<_, Option<String>>(1)?)),
                )?;
                self.lock()
                    .execute("DELETE FROM google_oauth_state WHERE state = ?1", [state])?;
                date
            }
        };

        Ok(OAuthState {
            created: res.0,
            redirect: res.1,
        })
    }
}
