use anyhow::Result;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use rusqlite::Connection;
use tracing::{error, info};

use crate::misc::current_epoch;

use self::{auth::AuthDb, misc::MiscDb, session::SessionDb, user::UserDb};

pub mod auth;
pub mod misc;
pub mod session;
pub mod user;

type SessionMeta = (String, u64, Option<String>);

// Increment every time schema changes, even in dev
const DATABASE_VERSION: u64 = 3;

pub struct Db {
    inner: Mutex<Option<Connection>>,
}

enum DbResult<T> {
    Ok(T),
    NotFound,
}

impl Db {
    pub fn new(connection: Connection) -> Self {
        Self {
            inner: Mutex::new(Some(connection)),
        }
    }

    fn take(&self) -> Connection {
        let val = self.inner.lock().take();
        val.expect("No value to take")
    }

    fn lock(&self) -> MappedMutexGuard<'_, Connection> {
        MutexGuard::map(self.inner.lock(), |x: &mut Option<Connection>| {
            x.as_mut().expect("No value to take")
        })
    }

    pub fn auth(&self) -> AuthDb {
        AuthDb(self)
    }

    pub fn session(&self) -> SessionDb {
        SessionDb(self)
    }

    pub fn user(&self) -> UserDb {
        UserDb(self)
    }

    pub fn misc(&self) -> MiscDb {
        MiscDb(self)
    }
}

impl Db {
    pub fn init(&self) -> Result<()> {
        let mut this = self.lock();
        this.pragma_update(None, "journal_mode", "WAL")?;
        this.pragma_update(None, "synchronous", "NORMAL")?;

        let db_version =
            this.pragma_query_value(None, "user_version", |row| row.get::<_, u64>(0))?;

        match db_version {
            DATABASE_VERSION => info!("Loaded database at `{}`", this.path().unwrap()),
            0 => {
                info!("Creating database at `{}`", this.path().unwrap());
                this.pragma_update(None, "user_version", DATABASE_VERSION)?;
            }
            i => {
                error!(
                    "Database version mismatch. Expected {}, got {}. Please run migrations, or \
                     just like delete the database and start over.",
                    DATABASE_VERSION, i
                );
            }
        }

        let trans = this.transaction()?;
        for i in [
            // == AUTH ==
            include_str!("./sql/auth/create_users.sql"),
            include_str!("./sql/auth/github/create_users.sql"),
            include_str!("./sql/auth/github/create_oauth_state.sql"),
            include_str!("./sql/auth/google/create_users.sql"),
            include_str!("./sql/auth/google/create_oauth_state.sql"),
            // == Sessions ==
            include_str!("./sql/session/create_sessions.sql"),
            // == Classes ==
            include_str!("./sql/class/create_class.sql"),
            include_str!("./sql/class/create_class_members.sql"),
            // == Problems ==
            include_str!("./sql/problems/create_problems.sql"),
            include_str!("./sql/problems/create_solutions.sql"),
        ] {
            trans.execute(i, [])?;
        }
        trans.commit()?;

        Ok(())
    }

    pub fn cleanup(&self) -> Result<()> {
        self.garbage_collect()?;

        let this = self.take();
        this.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        this.pragma_update(None, "optimize", "")?;
        this.pragma_update(None, "wal_checkpoint", "TRUNCATE")?;
        drop(this);

        Ok(())
    }

    fn garbage_collect(&self) -> anyhow::Result<()> {
        let mut this = self.lock();

        let cutoff = current_epoch() - 60 * 60; // (one hour)
        let trans = this.transaction()?;

        for i in [
            include_str!("./sql/auth/github/delete_oauth.sql"),
            include_str!("./sql/auth/google/delete_oauth.sql"),
        ] {
            trans.execute(i, [cutoff])?;
        }
        trans.commit()?;

        Ok(())
    }
}

trait SimplifyDbResult<T> {
    fn simplify(self) -> Result<DbResult<T>, rusqlite::Error>;
}

impl<T> SimplifyDbResult<T> for Result<T, rusqlite::Error> {
    fn simplify(self) -> Result<DbResult<T>, rusqlite::Error> {
        match self {
            Ok(t) => Ok(DbResult::Ok(t)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(DbResult::NotFound),
            Err(e) => Err(e),
        }
    }
}
