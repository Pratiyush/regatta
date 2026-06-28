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

/// One recorded cost event — a session's usage turn priced in USD.
#[derive(Debug, Clone, PartialEq)]
pub struct CostEvent {
    pub session_id: String,
    pub project: String,
    pub model: String,
    pub ts: i64,
    pub cost_usd: f64,
}

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS sessions (\
    id TEXT PRIMARY KEY, project TEXT NOT NULL, name TEXT NOT NULL, status TEXT NOT NULL);\
    CREATE TABLE IF NOT EXISTS transcript_index (\
    session_id TEXT PRIMARY KEY, file_path TEXT NOT NULL, last_offset INTEGER NOT NULL, \
    project TEXT NOT NULL, title TEXT NOT NULL, last_activity INTEGER NOT NULL);\
    CREATE INDEX IF NOT EXISTS idx_transcript_activity ON transcript_index(project, last_activity DESC);\
    CREATE TABLE IF NOT EXISTS cost_events (\
    id INTEGER PRIMARY KEY AUTOINCREMENT, session_id TEXT NOT NULL, project TEXT NOT NULL, \
    model TEXT NOT NULL, ts INTEGER NOT NULL, cost_usd REAL NOT NULL);\
    CREATE INDEX IF NOT EXISTS idx_cost_ts ON cost_events(ts);";

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

    /// Record a priced usage event.
    pub fn record_cost(&self, e: &CostEvent) -> rusqlite::Result<()> {
        self.conn.execute(
            "INSERT INTO cost_events (session_id, project, model, ts, cost_usd) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![e.session_id, e.project, e.model, e.ts, e.cost_usd],
        )?;
        Ok(())
    }

    /// Total spend across all recorded cost events.
    pub fn total_spend(&self) -> rusqlite::Result<f64> {
        self.conn.query_row(
            "SELECT COALESCE(SUM(cost_usd), 0.0) FROM cost_events",
            [],
            |r| r.get(0),
        )
    }

    /// Spend since a unix-second timestamp (inclusive).
    pub fn spend_since(&self, ts: i64) -> rusqlite::Result<f64> {
        self.conn.query_row(
            "SELECT COALESCE(SUM(cost_usd), 0.0) FROM cost_events WHERE ts >= ?1",
            params![ts],
            |r| r.get(0),
        )
    }

    /// Spend grouped by project, most-spent first.
    pub fn spend_by_project(&self) -> rusqlite::Result<Vec<(String, f64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT project, SUM(cost_usd) FROM cost_events GROUP BY project ORDER BY SUM(cost_usd) DESC",
        )?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
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

    #[test]
    fn board_query_resists_sql_injection() {
        let s = Store::open_in_memory().unwrap();
        s.upsert_transcript(&tx("s1", "payments", "real", 100))
            .unwrap();
        // a classic injection payload must be a literal search string, not executed SQL
        let hits = s
            .board_query("x'; DROP TABLE transcript_index; --")
            .unwrap();
        assert!(hits.is_empty(), "injection string should match nothing");
        // the table must survive and still be queryable
        let all = s.board_query("").unwrap();
        assert_eq!(
            all.len(),
            1,
            "transcript_index must survive the injection attempt"
        );
    }

    fn ce(session: &str, project: &str, cost: f64, ts: i64) -> CostEvent {
        CostEvent {
            session_id: session.into(),
            project: project.into(),
            model: "opus".into(),
            ts,
            cost_usd: cost,
        }
    }

    #[test]
    fn records_and_rolls_up_cost() {
        let s = Store::open_in_memory().unwrap();
        s.record_cost(&ce("a", "payments", 1.50, 100)).unwrap();
        s.record_cost(&ce("b", "payments", 0.50, 200)).unwrap();
        s.record_cost(&ce("c", "book", 3.00, 300)).unwrap();
        assert!((s.total_spend().unwrap() - 5.0).abs() < 1e-9);
        let by = s.spend_by_project().unwrap();
        assert_eq!(by[0], ("book".to_string(), 3.0)); // most-spent first
        assert_eq!(by[1], ("payments".to_string(), 2.0));
        assert!((s.spend_since(250).unwrap() - 3.0).abs() < 1e-9); // only the ts=300 event
    }

    #[test]
    fn empty_cost_rollups_are_zero() {
        let s = Store::open_in_memory().unwrap();
        assert_eq!(s.total_spend().unwrap(), 0.0);
        assert!(s.spend_by_project().unwrap().is_empty());
        assert_eq!(s.spend_since(0).unwrap(), 0.0);
    }
}
