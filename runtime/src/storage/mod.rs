use anyhow::Result;
use rusqlite::{Connection, params};
use tracing::info;

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn open(path: &str) -> Result<Self> {
        info!("Storage: opening {path}");
        let conn = Connection::open(path)?;
        let s = Self { conn };
        s.migrate()?;
        Ok(s)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let s = Self { conn };
        s.migrate()?;
        Ok(s)
    }

    fn migrate(&self) -> Result<()> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS kv (
                key   TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS apps (
                id           TEXT PRIMARY KEY,
                name         TEXT NOT NULL,
                version      TEXT NOT NULL,
                path         TEXT NOT NULL,
                installed_at INTEGER NOT NULL DEFAULT (unixepoch())
            );
        ")?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT value FROM kv WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;
        Ok(rows.next()?.map(|r| r.get(0)).transpose()?)
    }

    pub fn set(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO kv (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }
}
