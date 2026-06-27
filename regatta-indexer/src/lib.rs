//! Index Claude transcript files (`~/.claude/projects/**/*.jsonl`) into the flow store so the
//! Resume board can list every past session — reading only each file's head, never the whole 44 MB.

use regatta_core::transcript::{parse_session_meta, SessionMeta};
use regatta_store::{Store, TranscriptRow};
use std::io::Read;
use std::path::Path;

/// Scan `root` recursively for `.jsonl` transcripts and upsert each into the store's transcript
/// index. Returns how many were indexed. Unreadable files and non-transcript lines are skipped.
pub fn index_dir(store: &Store, root: &Path) -> std::io::Result<usize> {
    let mut count = 0;
    index_recursive(store, root, &mut count)?;
    Ok(count)
}

fn index_recursive(store: &Store, dir: &Path, count: &mut usize) -> std::io::Result<()> {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(()), // missing/unreadable dir → index nothing
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            index_recursive(store, &path, count)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("jsonl") {
            if let Some(row) = index_file(&path) {
                let _ = store.upsert_transcript(&row);
                *count += 1;
            }
        }
    }
    Ok(())
}

/// Read only the file's head and build an index row. The cwd isn't always on line 1, so take the
/// first line that parses as session metadata, preferring one that carries a working directory.
fn index_file(path: &Path) -> Option<TranscriptRow> {
    let mut f = std::fs::File::open(path).ok()?;
    let mut buf = vec![0u8; 64 * 1024];
    let n = f.read(&mut buf).ok()?;
    let head = String::from_utf8_lossy(&buf[..n]);
    let mut meta: Option<SessionMeta> = None;
    for line in head.lines() {
        if let Some(m) = parse_session_meta(line) {
            if meta.is_none() {
                meta = Some(m.clone());
            }
            if !m.cwd.is_empty() {
                meta = Some(m);
                break;
            }
        }
    }
    let meta = meta?;
    let last_activity = std::fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    Some(TranscriptRow {
        file_path: path.to_string_lossy().into_owned(),
        last_offset: 0,
        title: title_for(&meta),
        project: meta.project,
        session_id: meta.session_id,
        last_activity,
    })
}

fn title_for(meta: &SessionMeta) -> String {
    let short: String = meta.session_id.chars().take(8).collect();
    if meta.project.is_empty() || meta.project == "session" {
        format!("session {short}")
    } else {
        format!("{} · {short}", meta.project)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_a_transcript_dir() {
        let dir = std::env::temp_dir().join(format!("regatta_idx_{}", std::process::id()));
        let proj = dir.join("nested");
        std::fs::create_dir_all(&proj).unwrap();
        std::fs::write(
            proj.join("sess-a.jsonl"),
            "{\"sessionId\":\"sess-a\",\"cwd\":\"/Users/p/payments\",\"gitBranch\":\"main\",\"type\":\"user\"}\n",
        )
        .unwrap();
        std::fs::write(
            proj.join("sess-b.jsonl"),
            "{\"sessionId\":\"sess-b\",\"cwd\":\"/Users/p/book\",\"type\":\"user\"}\n",
        )
        .unwrap();
        std::fs::write(proj.join("garbage.jsonl"), "not json\n").unwrap();

        let store = Store::open_in_memory().unwrap();
        let n = index_dir(&store, &dir).unwrap();
        assert_eq!(n, 2, "two valid transcripts; the garbage file is skipped");
        let all = store.board_query("").unwrap();
        assert_eq!(all.len(), 2);
        assert!(all
            .iter()
            .any(|r| r.session_id == "sess-a" && r.project == "payments"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn missing_dir_yields_nothing() {
        let store = Store::open_in_memory().unwrap();
        let n = index_dir(&store, Path::new("/no/such/regatta/dir/xyz")).unwrap();
        assert_eq!(n, 0);
    }
}
