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

/// One indexed transcript (a past or live session) for the Resume board.
#[derive(Debug, Clone, PartialEq)]
pub struct TranscriptRow {
    pub session_id: String,
    pub file_path: String,
    pub last_offset: i64,
    pub project: String,
    pub title: String,
    pub last_activity: i64,
}

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS sessions (\
    id TEXT PRIMARY KEY, project TEXT NOT NULL, name TEXT NOT NULL, status TEXT NOT NULL);\
    CREATE TABLE IF NOT EXISTS transcript_index (\
    session_id TEXT PRIMARY KEY, file_path TEXT NOT NULL, last_offset INTEGER NOT NULL, \
    project TEXT NOT NULL, title TEXT NOT NULL, last_activity INTEGER NOT NULL);\
    CREATE INDEX IF NOT EXISTS idx_transcript_activity ON transcript_index(project, last_activity DESC);";

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

    /// Insert or update a transcript-index row (keyed by session id).
    pub fn upsert_transcript(&self, t: &TranscriptRow) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO transcript_index (session_id, file_path, last_offset, project, title, last_activity) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6) \
             ON CONFLICT(session_id) DO UPDATE SET file_path = ?2, last_offset = ?3, project = ?4, title = ?5, last_activity = ?6",
            params![t.session_id, t.file_path, t.last_offset, t.project, t.title, t.last_activity],
        )?;
        Ok(())
    }

    /// Resume-board search: rows whose title or project match `text`, most-recent first.
    pub fn board_query(&self, text: &str) -> rusqlite::Result<Vec<TranscriptRow>> {
        let like = format!("%{text}%");
        let mut stmt = self.conn.prepare(
            "SELECT session_id, file_path, last_offset, project, title, last_activity \
             FROM transcript_index WHERE title LIKE ?1 OR project LIKE ?1 \
             ORDER BY last_activity DESC",
        )?;
        let rows = stmt.query_map(params![like], |r| {
            Ok(TranscriptRow {
                session_id: r.get(0)?,
                file_path: r.get(1)?,
                last_offset: r.get(2)?,
                project: r.get(3)?,
                title: r.get(4)?,
                last_activity: r.get(5)?,
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

    fn tx(id: &str, project: &str, title: &str, activity: i64) -> TranscriptRow {
        TranscriptRow {
            session_id: id.into(),
            file_path: format!("/p/{id}.jsonl"),
            last_offset: 0,
            project: project.into(),
            title: title.into(),
            last_activity: activity,
        }
    }

    #[test]
    fn transcript_index_persists_and_searches() {
        let s = Store::open_in_memory().unwrap();
        s.upsert_transcript(&tx("s1", "payments", "Fix idempotency", 100))
            .unwrap();
        s.upsert_transcript(&tx("s2", "book-java", "Chapter 4 tests", 200))
            .unwrap();
        let all = s.board_query("").unwrap();
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].session_id, "s2"); // most-recent first
        let hits = s.board_query("idempo").unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].session_id, "s1");
    }

    #[test]
    fn upsert_transcript_updates_in_place() {
        let s = Store::open_in_memory().unwrap();
        s.upsert_transcript(&tx("s1", "p", "old title", 100))
            .unwrap();
        s.upsert_transcript(&tx("s1", "p", "new title", 150))
            .unwrap();
        let all = s.board_query("").unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].title, "new title");
        assert_eq!(all[0].last_activity, 150);
    }
}
