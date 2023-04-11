use rusqlite::{params, Connection};

use crate::{
    misc::{current_epoch, LoginProvider},
    session::{Session, SessionPlatform},
};

pub trait Database {
    // == Base ==
    fn init(&mut self);
    fn cleanup(&mut self);
    fn garbage_collect(&mut self);

    // == Auth ==
    fn add_oauth(&self, service: LoginProvider, state: &str) -> anyhow::Result<()>;
    fn get_oauth(&self, service: LoginProvider, state: &str) -> anyhow::Result<u64>;

    // == Session ==
    fn add_session(&self, session: &Session) -> anyhow::Result<()>;
    fn get_session(&self, token: &str) -> anyhow::Result<Session>;
}

impl Database for Connection {
    fn init(&mut self) {
        self.pragma_update(None, "journal_mode", "WAL").unwrap();
        self.pragma_update(None, "synchronous", "NORMAL").unwrap();

        let trans = self.transaction().unwrap();
        for i in [
            include_str!("./sql/auth/github/create_users.sql"),
            include_str!("./sql/auth/github/create_oauth_state.sql"),
            include_str!("./sql/auth/google/create_users.sql"),
            include_str!("./sql/auth/google/create_oauth_state.sql"),
            include_str!("./sql/create_sessions.sql"),
        ] {
            trans.execute(i, []).unwrap();
        }
        trans.commit().unwrap();
    }

    fn cleanup(&mut self) {
        self.pragma_update(None, "wal_checkpoint", "TRUNCATE")
            .unwrap();
    }

    fn garbage_collect(&mut self) {
        let cutoff = current_epoch() - 60 * 60; // (one hour)
        let trans = self.transaction().unwrap();

        for i in [
            include_str!("./sql/auth/github/delete_oauth.sql"),
            include_str!("./sql/auth/google/delete_oauth.sql"),
        ] {
            trans.execute(i, [cutoff]).unwrap();
        }
        trans.commit().unwrap();
    }

    fn add_oauth(&self, service: LoginProvider, state: &str) -> anyhow::Result<()> {
        match service {
            LoginProvider::Github => self.execute(
                "INSERT INTO github_oauth_state (state, created) VALUES (?1, strftime('%s','now'))",
                [state],
            ),
            LoginProvider::Google => self.execute(
                "INSERT INTO google_oauth_state (state, created) VALUES (?1, strftime('%s','now'))",
                [state],
            ),
        }?;

        Ok(())
    }

    /// Gets and removes the oauth state
    fn get_oauth(&self, service: LoginProvider, state: &str) -> anyhow::Result<u64> {
        let res = match service {
            LoginProvider::Github => {
                let date = self.query_row(
                    "SELECT created FROM github_oauth_state WHERE state = ?1",
                    [state],
                    |x| x.get::<_, u64>(0),
                )?;
                self.execute("DELETE FROM github_oauth_state WHERE state = ?1", [state])?;
                date
            }
            LoginProvider::Google => {
                let date = self.query_row(
                    "SELECT created FROM google_oauth_state WHERE state = ?1",
                    [state],
                    |x| x.get::<_, u64>(0),
                )?;
                self.execute("DELETE FROM google_oauth_state WHERE state = ?1", [state])?;
                date
            }
        };

        Ok(res)
    }

    fn add_session(&self, session: &Session) -> anyhow::Result<()> {
        match &session.platform {
            SessionPlatform::Github(p) => {
                self.execute(
                    include_str!("./sql/auth/github/upsert_login.sql"),
                    params![
                        session.id,
                        p.github_id,
                        session.name,
                        p.login,
                        session.avatar,
                        session.token
                    ],
                )?;
            }
            SessionPlatform::Google(_) => todo!(),
        }

        Ok(())
    }

    fn get_session(&self, token: &str) -> anyhow::Result<Session> {
        todo!()
    }
}
