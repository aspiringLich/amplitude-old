use std::result;

use anyhow::Result;
use derive_more::{Deref, Display};
use serde::Serialize;

use super::Db;

#[derive(Deref)]
pub struct UserDb<'a>(pub(super) &'a Db);

#[derive(Debug, Serialize)]
pub enum ClassJoinError {
    AlreadyInClass,
    ClassDoesNotExist,
}

impl<'a> UserDb<'a> {
    pub fn join_class(&self, user_id: &str, class_id: u64) -> result::Result<(), ClassJoinError> {
        // Verify that the user is not already in the class
        // Verify that the class exists

        Ok(())
    }
}
