use rusqlite::Connection;

pub trait Database {
    // == Base ==
    fn init(&self);
    fn cleanup(&self);
}

impl Database for Connection {
    fn init(&self) {
        // Init Tables
        self.pragma_update(None, "journal_mode", "WAL").unwrap();
        self.pragma_update(None, "synchronous", "NORMAL").unwrap();
    }

    fn cleanup(&self) {
        self.pragma_update(None, "wal_checkpoint", "TRUNCATE")
            .unwrap();
    }
}
