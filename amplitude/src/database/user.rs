use std::result;

use derive_more::Deref;
use rusqlite::params;
use serde::Serialize;

use super::Db;

#[derive(Deref)]
pub struct UserDb<'a>(pub(super) &'a Db);

#[derive(Debug, Serialize)]
pub enum ClassJoinError {
    AlreadyInClass,
    ClassDoesNotExist,
    #[serde(skip)]
    DatabaseError(rusqlite::Error),
}

impl<'a> UserDb<'a> {
    pub fn join_class(&self, user_id: &str, class_id: u64) -> result::Result<(), ClassJoinError> {
        let this = self.lock();

        // Verify that the class exists
        match this.query_row(
            "SELECT EXISTS (SELECT 1 FROM class WHERE id = ?)",
            [class_id],
            |row| row.get::<_, bool>(1),
        ) {
            Ok(true) => {}
            Ok(false) => return Err(ClassJoinError::ClassDoesNotExist),
            Err(e) => return Err(ClassJoinError::ClassDoesNotExist),
        };

        // Verify that the user is not already in the class
        match this.query_row(
            "SELECT EXISTS (SELECT 1 FROM class_members WHERE user_id = ? AND class_id = ?)",
            params![user_id, class_id],
            |row| row.get::<_, bool>(1),
        ) {
            Ok(true) => return Err(ClassJoinError::AlreadyInClass),
            Ok(false) => {}
            Err(e) => return Err(ClassJoinError::AlreadyInClass),
        };

        // Add user to class
        this.execute(
            "INSERT INTO class_members (user_id, class_id, date_added) VALUES (?, ?, strftime('%s', 'now'))",
            params![user_id, class_id],
        )?;

        Ok(())
    }
}

impl From<rusqlite::Error> for ClassJoinError {
    fn from(e: rusqlite::Error) -> Self {
        Self::DatabaseError(e)
    }
}
