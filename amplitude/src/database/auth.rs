use crate::misc::LoginProvider;

use super::Db;
use anyhow::Result;
use derive_more::Deref;

#[derive(Deref)]
pub struct AuthDb<'a>(pub(super) &'a Db);

impl<'a> AuthDb<'a> {
    pub fn add_oauth(&self, service: LoginProvider, state: &str) -> Result<()> {
        match service {
            LoginProvider::Github => self.lock().execute(
                "INSERT INTO github_oauth_state (state, created) VALUES (?1, strftime('%s','now'))",
                [state],
            ),
            LoginProvider::Google => self.lock().execute(
                "INSERT INTO google_oauth_state (state, created) VALUES (?1, strftime('%s','now'))",
                [state],
            ),
        }?;

        Ok(())
    }

    /// Gets and removes the oauth state
    pub fn get_oauth(&self, service: LoginProvider, state: &str) -> Result<u64> {
        let res = match service {
            LoginProvider::Github => {
                let date = self.lock().query_row(
                    "SELECT created FROM github_oauth_state WHERE state = ?1",
                    [state],
                    |x| x.get::<_, u64>(0),
                )?;
                self.lock()
                    .execute("DELETE FROM github_oauth_state WHERE state = ?1", [state])?;
                date
            }
            LoginProvider::Google => {
                let date = self.lock().query_row(
                    "SELECT created FROM google_oauth_state WHERE state = ?1",
                    [state],
                    |x| x.get::<_, u64>(0),
                )?;
                self.lock()
                    .execute("DELETE FROM google_oauth_state WHERE state = ?1", [state])?;
                date
            }
        };

        Ok(res)
    }
}
