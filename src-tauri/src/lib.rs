//! Thin Tauri glue over `regatta_core`. Logic belongs in the pure core; this only wires it.

use regatta_core::attention::{attention_priority, dock_order, SessionState};
use serde::Serialize;

/// Returns a worktree-safe slug for a session name, computed by the pure core.
#[tauri::command]
fn slugify(name: &str) -> String {
    regatta_core::slug::slugify(name)
}

#[derive(Serialize, Clone)]
struct SessionView {
    id: String,
    name: String,
    project: String,
    branch: String,
    status: String,
    status_label: String,
    priority: u8,
    action: String,
    cost: String,
    reason: String,
}

#[derive(Serialize)]
struct DockView {
    sessions: Vec<SessionView>,
    /// Indices into `sessions`, most-urgent first — computed by the pure core.
    order: Vec<usize>,
}

fn status_str(s: SessionState) -> (&'static str, &'static str) {
    use SessionState::*;
    match s {
        WaitingApproval => ("waiting-approval", "Needs approval"),
        NeedsInput => ("needs-input", "Needs you"),
        Error => ("error", "Error"),
        Done => ("done", "Done"),
        Running => ("running", "Working"),
        Starting => ("starting", "Starting"),
        Compacting => ("compacting", "Compacting"),
        RateLimited => ("rate-limited", "Rate-limited"),
        Paused => ("paused", "Paused"),
    }
}

/// Mock cockpit data for the M1 shell — real sessions arrive with the supervisor (later M1 slice).
/// The Attention Dock ordering is computed by the pure core (`regatta_core::attention`).
#[tauri::command]
fn dock_view() -> DockView {
    // (project, name, branch, state, seen, action, cost, reason)
    let raw: Vec<(&str, &str, &str, SessionState, bool, &str, &str, &str)> = vec![
        ("payments-svc", "fix-idempotency", "fix/idempotency", SessionState::WaitingApproval, false, "run a production migration", "$2.41", "Writes to the production database"),
        ("Book-Java", "ch04-tests", "main", SessionState::Running, false, "writing chapter 4 tests", "$0.88", ""),
        ("patent", "explore-claims", "explore", SessionState::NeedsInput, false, "asks: which claim scope?", "$1.12", "Needs a decision to continue"),
        ("Quarkus", "fix-foundation", "fix/foundation", SessionState::Error, false, "build failed: NullPointerException", "$0.31", "Build is red"),
        ("notepad++", "scintilla", "main", SessionState::Running, false, "editing ScintillaEditView.cpp", "$1.77", ""),
        ("regatta", "attention-dock", "feat/attention-dock", SessionState::Done, false, "opened PR #6", "$0.54", ""),
        ("temp", "scratch", "main", SessionState::Running, false, "grepping logs", "$0.09", ""),
    ];
    let states: Vec<(SessionState, bool)> = raw.iter().map(|r| (r.3, r.4)).collect();
    let order = dock_order(&states);
    let sessions = raw
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let (status, label) = status_str(r.3);
            SessionView {
                id: format!("s{}", i + 1),
                name: r.1.to_string(),
                project: r.0.to_string(),
                branch: r.2.to_string(),
                status: status.to_string(),
                status_label: label.to_string(),
                priority: attention_priority(r.3),
                action: r.5.to_string(),
                cost: r.6.to_string(),
                reason: r.7.to_string(),
            }
        })
        .collect();
    DockView { sessions, order }
}

/// Run a deterministic demo session through the REAL pipeline (supervisor → parser → view) using a
/// fake stream-json backend — no Claude auth, no tokens. Real `claude` is the same launch plan.
#[tauri::command]
async fn run_demo_session() -> Vec<regatta_core::view::EventLine> {
    let script = "echo '{\"type\":\"system\",\"subtype\":\"init\",\"model\":\"claude-opus-4-8\"}'; \
                  echo '{\"type\":\"assistant\",\"message\":{\"content\":[{\"type\":\"text\",\"text\":\"Reading the failing test in payments/idempotency_test.rs\"}]}}'; \
                  echo '{\"type\":\"assistant\",\"message\":{\"content\":[{\"type\":\"text\",\"text\":\"The webhook handler is not deduping on event id. Adding an idempotency guard.\"}]}}'; \
                  echo '{\"type\":\"assistant\",\"message\":{\"content\":[{\"type\":\"text\",\"text\":\"Done. Ran the suite: 14 passed.\"}]}}'; \
                  echo '{\"type\":\"result\",\"total_cost_usd\":0.42,\"usage\":{\"input_tokens\":18400,\"output_tokens\":2100}}'";
    let plan = regatta_core::backend::LaunchPlan {
        program: "/bin/sh".into(),
        args: vec!["-c".into(), script.into()],
        env: Vec::new(),
        cwd: std::path::PathBuf::from("/"),
    };
    match regatta_supervisor::SessionHandle::spawn(&plan) {
        Ok(mut h) => {
            let events = h.collect_events().await;
            h.shutdown().await;
            events.iter().map(regatta_core::view::event_line).collect()
        }
        Err(_) => Vec::new(),
    }
}

#[derive(Serialize, Clone)]
struct BoardRow {
    session_id: String,
    project: String,
    title: String,
    group: String,
    resume_cmd: String,
}

/// Index `~/.claude/projects` into `store` — used once at startup and again on Re-index.
fn index_claude_home(store: &regatta_store::Store) {
    let home = std::env::var("HOME").unwrap_or_default();
    let dir = std::path::Path::new(&home).join(".claude/projects");
    let _ = regatta_indexer::index_dir(store, &dir);
}

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// The Resume board: indexed sessions matching `query`, grouped by recency (pure core), each with a
/// copy-paste `claude --resume <id>` command. Queries the in-memory index built at startup.
#[tauri::command]
fn board_list(
    query: String,
    store: tauri::State<'_, std::sync::Mutex<regatta_store::Store>>,
) -> Vec<BoardRow> {
    let now = now_secs();
    let Ok(store) = store.lock() else {
        return Vec::new();
    };
    store
        .board_query(&query)
        .unwrap_or_default()
        .into_iter()
        .take(80)
        .map(|r| BoardRow {
            group: regatta_core::board::recency_group(r.last_activity, now).to_string(),
            resume_cmd: regatta_core::backend::resume_command(&r.session_id),
            session_id: r.session_id,
            project: r.project,
            title: r.title,
        })
        .collect()
}

/// Re-scan `~/.claude/projects` into the in-memory index (the board's Re-index button).
#[tauri::command]
fn board_reindex(store: tauri::State<'_, std::sync::Mutex<regatta_store::Store>>) {
    if let Ok(store) = store.lock() {
        index_claude_home(&store);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::Manager;
            let store = regatta_store::Store::open_in_memory().expect("flow store");
            index_claude_home(&store); // index the session history once at launch
            app.manage(std::sync::Mutex::new(store));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            slugify,
            dock_view,
            run_demo_session,
            board_list,
            board_reindex
        ])
        .run(tauri::generate_context!())
        .expect("error while running Regatta");
}
