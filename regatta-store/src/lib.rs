//! The flow store: durable session metadata in SQLite, so sessions survive a restart
//! and the Resume board can list them.

use rusqlite::{params, Connection};

/// One persisted session row.
#[derive(Debug, Clone, PartialEq)]
pub struct SessionRow {
    pub id: String,
    pub project: String,
    pub name: String,
    pub status: String,
}

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS sessions (\
    id TEXT PRIMARY KEY, project TEXT NOT NULL, name TEXT NOT NULL, status TEXT NOT NULL);";

/// A handle to the flow store.
pub struct Store {
    conn: Connection,
}

impl Store {
    /// Open (or create) the store at `path`.
    pub fn open(path: &str) -> rusqlite::Result<Store> {
        let conn = Connection::open(path)?;
        conn.execute_batch(SCHEMA)?;
        Ok(Store { conn })
    }

    /// An ephemeral in-memory store (for tests).
    pub fn open_in_memory() -> rusqlite::Result<Store> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA)?;
        Ok(Store { conn })
    }

    /// Insert a session, or update it in place if the id already exists (never duplicates).
    pub fn upsert_session(&self, s: &SessionRow) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO sessions (id, project, name, status) VALUES (?1, ?2, ?3, ?4) \
             ON CONFLICT(id) DO UPDATE SET project = ?2, name = ?3, status = ?4",
            params![s.id, s.project, s.name, s.status],
        )?;
        Ok(())
    }

    /// All sessions, ordered by id.
    pub fn list_sessions(&self) -> rusqlite::Result<Vec<SessionRow>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, project, name, status FROM sessions ORDER BY id")?;
        let rows = stmt.query_map([], |r| {
            Ok(SessionRow {
                id: r.get(0)?,
                project: r.get(1)?,
                name: r.get(2)?,
                status: r.get(3)?,
            })
        })?;
        rows.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(id: &str, status: &str) -> SessionRow {
        SessionRow {
            id: id.into(),
            project: "payments".into(),
            name: "fix".into(),
            status: status.into(),
        }
    }

    #[test]
    fn persists_and_lists_sessions() {
        let store = Store::open_in_memory().unwrap();
        store.upsert_session(&row("s1", "running")).unwrap();
        store.upsert_session(&row("s2", "done")).unwrap();
        let rows = store.list_sessions().unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], row("s1", "running"));
    }

    #[test]
    fn upsert_updates_existing_session() {
        let store = Store::open_in_memory().unwrap();
        store.upsert_session(&row("s1", "running")).unwrap();
        store.upsert_session(&row("s1", "done")).unwrap(); // same id -> update in place
        let rows = store.list_sessions().unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].status, "done");
    }
}
