use rusqlite::Connection;

pub trait Database {
    // == Base ==
    fn init(&mut self);
    fn cleanup(&self);
}

impl Database for Connection {
    fn init(&mut self) {
        self.pragma_update(None, "journal_mode", "WAL").unwrap();
        self.pragma_update(None, "synchronous", "NORMAL").unwrap();

        let trans = self.transaction().unwrap();
        for i in [
            include_str!("./sql/create_github_users.sql"),
            include_str!("./sql/create_google_users.sql"),
        ] {
            trans.execute(i, []).unwrap();
        }
        trans.commit().unwrap();
    }

    fn cleanup(&self) {
        self.pragma_update(None, "wal_checkpoint", "TRUNCATE")
            .unwrap();
    }
}
