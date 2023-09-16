use std::result;

use anyhow::Result;
use derive_more::Deref;
use rusqlite::params;

use self::class::{class_exists, in_class, ClassError, MemberClass};
use super::Db;

#[derive(Deref)]
pub struct UserDb<'a>(pub(super) &'a Db);

impl<'a> UserDb<'a> {
    pub fn join_class(&self, user_id: &str, class_id: u64) -> result::Result<(), ClassError> {
        // Verify that the class exists
        class_exists(self, class_id)?;

        // Verify that the user is not already in the class
        if in_class(self, user_id, class_id)? {
            return Err(ClassError::AlreadyInClass);
        }

        // Add user to class
        let this = self.lock();
        this.execute(
            "INSERT INTO class_members (user_id, class_id, date_added) VALUES (?, ?, \
             strftime('%s', 'now'))",
            params![user_id, class_id],
        )?;

        Ok(())
    }

    pub fn leave_class(&self, user_id: &str, class_id: u64) -> result::Result<(), ClassError> {
        // Verify that the class exists
        class_exists(self, class_id)?;

        // Verify that the user is in class
        if !in_class(self, user_id, class_id)? {
            return Err(ClassError::NotInClass);
        }

        // Remove user from class
        let this = self.lock();
        this.execute(
            "DELETE FROM class_members WHERE user_id = ? AND class_id = ?",
            params![user_id, class_id],
        )?;

        Ok(())
    }

    pub fn list_classes(&self, user_id: &str) -> Result<Vec<MemberClass>> {
        let this = self.lock();
        let mut stmt = this.prepare(
            "SELECT class.id, class.name, class_members.date_added FROM class JOIN class_members \
             ON class.id = class_members.class_id WHERE class_members.user_id = ?",
        )?;
        let mut rows = stmt.query(params![user_id])?;

        let mut classes = Vec::new();
        while let Some(row) = rows.next()? {
            classes.push(MemberClass {
                class_id: row.get(0)?,
                class_name: row.get(1)?,
                date_joined: row.get(2)?,
            });
        }

        Ok(classes)
    }
}

pub mod class {
    use std::result;

    use rusqlite::params;
    use serde::Serialize;

    use super::UserDb;
    use crate::database::{DbResult, SimplifyDbResult};

    #[derive(Debug, Serialize)]
    pub enum ClassError {
        NotInClass,
        AlreadyInClass,
        ClassDoesNotExist,
        #[serde(skip)]
        DatabaseError(rusqlite::Error),
    }

    #[derive(Debug, Serialize)]
    pub struct MemberClass {
        pub class_id: u64,
        pub date_joined: u64,
        pub class_name: String,
    }

    impl From<rusqlite::Error> for ClassError {
        fn from(e: rusqlite::Error) -> Self {
            Self::DatabaseError(e)
        }
    }

    pub fn class_exists(db: &UserDb<'_>, class_id: u64) -> result::Result<(), ClassError> {
        let this = db.lock();
        if let DbResult::NotFound = this
            .query_row("SELECT * FROM class WHERE id = ?", [class_id], |_| Ok(()))
            .simplify()?
        {
            return Err(ClassError::ClassDoesNotExist);
        };

        Ok(())
    }

    pub fn in_class(
        db: &UserDb<'_>,
        user_id: &str,
        class_id: u64,
    ) -> result::Result<bool, rusqlite::Error> {
        let this = db.lock();
        match this
            .query_row(
                "SELECT * FROM class_members WHERE user_id = ? AND class_id = ?",
                params![user_id, class_id],
                |_| Ok(()),
            )
            .simplify()?
        {
            DbResult::Ok(()) => Ok(true),
            DbResult::NotFound => Ok(false),
        }
    }
}
