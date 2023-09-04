use super::Db;
use anyhow::{bail, Result};
use derive_more::Deref;
use rand::seq::SliceRandom;
use rusqlite::params;

#[derive(Deref)]
pub struct MiscDb<'a>(pub(super) &'a Db);

impl<'a> MiscDb<'a> {
    pub fn create_class(&self, id: u64, name: &str) -> Result<()> {
        self.lock().execute(
            "INSERT INTO class (id, name, created) VALUES (?1, ?2, strftime('%s','now'))",
            params![id, name],
        )?;

        Ok(())
    }

    pub fn class_name(&self, id: u64) -> Result<String> {
        let name =
            self.lock()
                .query_row("SELECT name FROM class WHERE id = ?1", params![id], |x| {
                    x.get::<_, String>(0)
                })?;

        Ok(name)
    }

    pub fn unique_class_id(&self) -> Result<u64> {
        let this = self.lock();

        let mut stmt = this.prepare("SELECT id from class")?;
        let mut rows = stmt.query([])?;

        let mut ids = Vec::new();
        while let Some(row) = rows.next()? {
            ids.push(row.get(0)?);
        }

        // surely this is a good idea
        let mut options = (1000..9999).collect::<Vec<_>>();
        options.shuffle(&mut rand::thread_rng());

        for id in options {
            if !ids.contains(&id) {
                return Ok(id);
            }
        }

        bail!("No unique class id found")
    }

    pub fn save_problem_progress(&self, user_id: &str, problem_id: u64, code: &str) -> Result<()> {
        // todo: validate problem_id

        self.lock().execute(
            include_str!("./sql/problems/upsert_problem.sql"),
            params![user_id, problem_id, code, false],
        )?;

        Ok(())
    }

    pub fn load_problem_progress(&self, user_id: &str, problem_id: u64) -> Result<String> {
        // todo: validate problem_id

        let code = self.lock().query_row(
            include_str!("./sql/problems/load_problem.sql"),
            params![user_id, problem_id],
            |x| x.get::<_, String>(0),
        )?;

        Ok(code)
    }
}
