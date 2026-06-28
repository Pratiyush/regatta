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
    backend: String,
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
                // a Claude/Codex mix — label via the proven pure dispatch
                backend: if matches!(i, 2 | 4) {
                    regatta_core::backend::Backend::Codex
                } else {
                    regatta_core::backend::Backend::Claude
                }
                .label()
                .to_string(),
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

#[derive(Serialize, Clone)]
struct SpendSlice {
    name: String,
    usd: f64,
}

#[derive(Serialize)]
struct UsageView {
    today_usd: f64,
    burn_per_hr: f64,
    budget_usd: f64,
    budget_pct: u8,
    by_project: Vec<SpendSlice>,
    by_model: Vec<SpendSlice>,
}

/// Seed representative cost events — demo data for the Usage view until live sessions record their own.
fn seed_cost_demo(store: &regatta_store::Store) {
    let now = now_secs();
    let demo: &[(&str, &str, &str, f64, i64)] = &[
        ("s1", "payments-svc", "claude-opus-4-8", 2.41, 0),
        ("s2", "Book-Java", "claude-sonnet-4-6", 0.88, 600),
        ("s3", "patent", "claude-opus-4-8", 1.12, 1200),
        ("s4", "Quarkus", "claude-haiku-4-5", 0.31, 1800),
        ("s5", "notepad++", "claude-opus-4-8", 1.77, 2400),
        ("s6", "regatta", "claude-sonnet-4-6", 0.54, 3000),
        ("s7", "temp", "claude-haiku-4-5", 0.09, 3600),
    ];
    for (sid, proj, model, cost, off) in demo {
        let _ = store.record_cost(&regatta_store::CostEvent {
            session_id: (*sid).into(),
            project: (*proj).into(),
            model: (*model).into(),
            ts: now - off,
            cost_usd: *cost,
        });
    }
}

/// The Usage view: spend today, burn rate, budget bar, and breakdowns by project and model —
/// aggregated from the cost ledger; budget percent + burn rate computed by the pure core.
#[tauri::command]
fn usage_view(store: tauri::State<'_, std::sync::Mutex<regatta_store::Store>>) -> UsageView {
    let now = now_secs();
    let budget_usd = 25.0;
    let zero = || UsageView {
        today_usd: 0.0,
        burn_per_hr: 0.0,
        budget_usd,
        budget_pct: 0,
        by_project: Vec::new(),
        by_model: Vec::new(),
    };
    let Ok(store) = store.lock() else {
        return zero();
    };
    let today = store.spend_since(now - 86_400).unwrap_or(0.0);
    let slice = |v: Vec<(String, f64)>| -> Vec<SpendSlice> {
        v.into_iter().map(|(name, usd)| SpendSlice { name, usd }).collect()
    };
    UsageView {
        today_usd: today,
        burn_per_hr: regatta_core::cost::burn_rate(today, 4 * 3600), // ~4h active window
        budget_usd,
        budget_pct: regatta_core::cost::budget_pct(today, budget_usd),
        by_project: slice(store.spend_by_project().unwrap_or_default()),
        by_model: slice(store.spend_by_model().unwrap_or_default()),
    }
}

#[derive(Serialize, Clone)]
struct ReviewItem {
    session: String,
    project: String,
    branch: String,
    files: usize,
    added: u64,
    removed: u64,
}

#[derive(Serialize, Clone)]
struct FileEntry {
    path: String,
    status: String,
    added: Option<u64>,
    removed: Option<u64>,
}

/// The Review Inbox: sessions with pending uncommitted changes, summarized. Demo data until live
/// sessions report their own; the real path is `regatta_git::status` + `regatta_core::git::summarize_diff`.
#[tauri::command]
fn review_inbox() -> Vec<ReviewItem> {
    let raw: &[(&str, &str, &str, usize, u64, u64)] = &[
        ("s1", "payments-svc", "fix/idempotency", 3, 84, 12),
        ("s2", "Book-Java", "ch04-tests", 5, 210, 4),
        ("s5", "notepad++", "scintilla", 2, 31, 9),
        ("s6", "regatta", "feat/review", 7, 156, 22),
    ];
    raw.iter()
        .map(|r| ReviewItem {
            session: r.0.into(),
            project: r.1.into(),
            branch: r.2.into(),
            files: r.3,
            added: r.4,
            removed: r.5,
        })
        .collect()
}

/// The diff drawer for a session: its changed files with status + per-file +/- (binary → no counts).
#[tauri::command]
fn diff_view(session: String) -> Vec<FileEntry> {
    let _ = session;
    let raw: &[(&str, &str, Option<u64>, Option<u64>)] = &[
        ("src/webhook.rs", "M", Some(42), Some(8)),
        ("src/idempotency.rs", "A", Some(36), Some(0)),
        ("tests/webhook_test.rs", "M", Some(6), Some(4)),
        ("assets/diagram.png", "A", None, None),
    ];
    raw.iter()
        .map(|r| FileEntry {
            path: r.0.into(),
            status: r.1.into(),
            added: r.2,
            removed: r.3,
        })
        .collect()
}

#[derive(Serialize, Clone)]
struct ConfigEntry {
    key: String,
    value: String,
    secret: bool,
}

#[derive(Serialize, Clone)]
struct Toggle {
    name: String,
    enabled: bool,
}

#[derive(Serialize, Clone)]
struct McpServer {
    name: String,
    tools: usize,
    enabled: bool,
}

#[derive(Serialize)]
struct SettingsView {
    config: Vec<ConfigEntry>,
    mcp_servers: Vec<McpServer>,
    skills: Vec<Toggle>,
    commands: Vec<String>,
}

/// The Settings + Extensions view: the effective layered config (secrets masked by the pure core)
/// plus MCP servers, skills, and commands. Demo layers until the config store is wired through.
#[tauri::command]
fn settings_view() -> SettingsView {
    use regatta_core::config::{effective_masked, is_secret_key, ConfigLayer};
    let global = ConfigLayer::from([
        ("model".to_string(), "claude-opus-4-8".to_string()),
        ("permission_mode".to_string(), "ask".to_string()),
        ("theme".to_string(), "regatta-dark".to_string()),
        (
            "env.ANTHROPIC_API_KEY".to_string(),
            "sk-ant-abcd1234efgh5678".to_string(),
        ),
    ]);
    let project = ConfigLayer::from([
        ("working_dir".to_string(), "~/Desktop/AI/terminal-x".to_string()),
        ("model".to_string(), "claude-opus-4-8".to_string()),
    ]);
    let config = effective_masked(&[global, project])
        .into_iter()
        .map(|(k, v)| ConfigEntry {
            secret: is_secret_key(&k),
            key: k,
            value: v,
        })
        .collect();
    let mcp = |name: &str, tools: usize, enabled: bool| McpServer {
        name: name.into(),
        tools,
        enabled,
    };
    let toggle = |name: &str, enabled: bool| Toggle {
        name: name.into(),
        enabled,
    };
    SettingsView {
        config,
        mcp_servers: vec![
            mcp("regatta", 4, true),
            mcp("github", 26, true),
            mcp("filesystem", 11, false),
            mcp("playwright", 21, true),
        ],
        skills: vec![
            toggle("dev-spec-kit", true),
            toggle("laptop-health", true),
            toggle("design-sync", false),
        ],
        commands: ["/plan", "/review", "/session", "/run", "/cost", "/resume"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::Manager;
            let store = regatta_store::Store::open_in_memory().expect("flow store");
            index_claude_home(&store); // index the session history once at launch
            seed_cost_demo(&store); // representative cost events for the Usage view
            app.manage(std::sync::Mutex::new(store));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            slugify,
            dock_view,
            run_demo_session,
            board_list,
            board_reindex,
            usage_view,
            review_inbox,
            diff_view,
            settings_view
        ])
        .run(tauri::generate_context!())
        .expect("error while running Regatta");
}
