//! Glue: spawn an agent process in its own process group, with guaranteed teardown of the whole tree.
//!
//! This is Regatta's reliability promise: when a session ends we kill the agent **and every child it
//! spawned**, leaving nothing behind — the failure mode that leaves cmux helpers spinning for days.

use regatta_core::backend::{Backend, LaunchPlan};
use regatta_core::budget::{budget_status, should_pause, Budget};
use std::process::Stdio;
use tokio::process::{Child, Command};

/// Owns a spawned agent process group. Dropping it force-kills the group (a panic-safe last resort).
pub struct SessionHandle {
    child: Child,
    /// Process-group id (equals the leader pid). `<= 0` when unavailable.
    pgid: i32,
}

impl SessionHandle {
    /// Spawn `plan` in a fresh process group, env-tagged, with piped stdio.
    pub fn spawn(plan: &LaunchPlan) -> std::io::Result<SessionHandle> {
        let mut std_cmd = std::process::Command::new(&plan.program);
        std_cmd
            .args(&plan.args)
            .current_dir(&plan.cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (k, v) in &plan.env {
            std_cmd.env(k, v);
        }
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            // New process group; the child becomes the group leader (pgid == child pid).
            std_cmd.process_group(0);
        }
        let mut cmd = Command::from(std_cmd);
        cmd.kill_on_drop(true);
        let child = cmd.spawn()?;
        let pgid = child.id().map(|id| id as i32).unwrap_or(-1);
        Ok(SessionHandle { child, pgid })
    }

    /// The leader pid (also the process-group id), if the process is still spawned.
    pub fn pid(&self) -> Option<u32> {
        self.child.id()
    }

    /// Read the agent's stdout to EOF as a Claude session (shorthand for `collect_events_with`).
    pub async fn collect_events(&mut self) -> Vec<regatta_core::stream::NormalizedEvent> {
        self.collect_events_with(Backend::Claude).await
    }

    /// Read the agent's stdout to EOF, parsing each line with `backend` into normalized events and
    /// skipping unparseable lines. The same spawn/collect/teardown path serves Claude and Codex.
    pub async fn collect_events_with(
        &mut self,
        backend: Backend,
    ) -> Vec<regatta_core::stream::NormalizedEvent> {
        let mut events = Vec::new();
        self.pump_events_with(backend, |ev| events.push(ev)).await;
        events
    }

    /// Read the agent's stdout to EOF, parsing each line with `backend` and invoking `on_event` for
    /// each event **as it arrives** — the live pump (vs `collect_events_with`, which batches). This is
    /// what feeds the live `Registry` and, in the UI, the per-session Channel.
    pub async fn pump_events_with<F>(&mut self, backend: Backend, mut on_event: F)
    where
        F: FnMut(regatta_core::stream::NormalizedEvent),
    {
        use tokio::io::{AsyncBufReadExt, BufReader};
        if let Some(stdout) = self.child.stdout.take() {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if let Some(ev) = backend.parse_line(&line) {
                    on_event(ev);
                }
            }
        }
    }

    /// Tear the session down: SIGTERM the whole group for a graceful exit, give it a short grace
    /// period, then SIGKILL the group — killing the agent and every child it spawned. Finally reap
    /// the leader. Idempotent and safe to call on an already-exited session.
    pub async fn shutdown(&mut self) {
        #[cfg(unix)]
        if self.pgid > 0 {
            unsafe {
                libc::killpg(self.pgid, libc::SIGTERM);
            }
            let _ = tokio::time::timeout(std::time::Duration::from_millis(1500), self.child.wait())
                .await;
            unsafe {
                libc::killpg(self.pgid, libc::SIGKILL);
            }
        }
        let _ = self.child.start_kill();
        let _ = self.child.wait().await;
    }

    /// Auto-pause: if `spent_usd` has crossed the budget ceiling and the action is Block, tear the
    /// session down (the runaway-stop cmux never had). Returns whether it paused.
    pub async fn autopause_if_exceeded(&mut self, spent_usd: f64, budget: &Budget) -> bool {
        if should_pause(budget_status(spent_usd, budget), budget.action) {
            self.shutdown().await;
            true
        } else {
            false
        }
    }
}

impl Drop for SessionHandle {
    fn drop(&mut self) {
        // A panic or early return must never leak the group.
        #[cfg(unix)]
        if self.pgid > 0 {
            unsafe {
                libc::killpg(self.pgid, libc::SIGKILL);
            }
        }
    }
}

/// Tail a transcript file from `from_offset` to EOF, parsing each complete line into events. A
/// trailing partial (newline-less) line is held until it completes; the returned offset is where to
/// resume next time, so reattach never re-reads the whole file. If the file is shorter than the
/// offset (truncated/rotated), reading restarts from the beginning.
pub fn tail_transcript(
    path: &std::path::Path,
    from_offset: u64,
) -> std::io::Result<(Vec<regatta_core::stream::NormalizedEvent>, u64)> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = std::fs::File::open(path)?;
    let len = f.metadata()?.len();
    // If the file is shorter than our offset, it was truncated/rotated — restart from the top.
    let start = if from_offset > len { 0 } else { from_offset };
    f.seek(SeekFrom::Start(start))?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    // Only consume up to the last newline; a trailing partial line is left for next time.
    let consumed = buf.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let events = buf[..consumed]
        .lines()
        .filter_map(regatta_core::stream::parse_claude_line)
        .collect();
    Ok((events, start + consumed as u64))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::Duration;

    fn sh(cmd: &str) -> LaunchPlan {
        LaunchPlan {
            program: "/bin/sh".into(),
            args: vec!["-c".into(), cmd.into()],
            env: Vec::new(),
            cwd: PathBuf::from("/"),
        }
    }

    fn alive(pid: i32) -> bool {
        unsafe { libc::kill(pid, 0) == 0 }
    }

    #[tokio::test]
    async fn shutdown_kills_the_whole_group() {
        // sh spawns a grandchild that records its own pid to a file, then both sleep.
        let pidfile = std::env::temp_dir().join(format!("regatta_gc_{}.pid", std::process::id()));
        let _ = std::fs::remove_file(&pidfile);
        let cmd = format!("sleep 30 & echo $! > {} ; sleep 30", pidfile.display());
        let mut h = SessionHandle::spawn(&sh(&cmd)).expect("spawn");

        // Wait until the grandchild has recorded its pid.
        let mut gc = 0i32;
        for _ in 0..100 {
            if let Ok(s) = std::fs::read_to_string(&pidfile) {
                if let Ok(p) = s.trim().parse::<i32>() {
                    gc = p;
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        assert!(gc > 0, "grandchild pid was not captured");
        assert!(alive(gc), "grandchild should be alive before shutdown");

        h.shutdown().await;
        tokio::time::sleep(Duration::from_millis(300)).await;

        assert!(
            !alive(gc),
            "shutdown must kill the grandchild, not just the agent"
        );
        let _ = std::fs::remove_file(&pidfile);
    }

    #[tokio::test]
    async fn shutdown_is_safe_when_already_exited() {
        let mut h = SessionHandle::spawn(&sh("exit 0")).expect("spawn");
        tokio::time::sleep(Duration::from_millis(150)).await; // let it exit on its own
        h.shutdown().await; // must not panic or error
        h.shutdown().await; // idempotent
    }

    #[tokio::test]
    async fn collects_parsed_events_from_stdout() {
        let script = "echo '{\"type\":\"system\",\"subtype\":\"init\",\"model\":\"m\"}'; \
                      echo 'garbage'; \
                      echo '{\"type\":\"assistant\",\"message\":{\"content\":[{\"type\":\"text\",\"text\":\"hi\"}]}}'; \
                      echo '{\"type\":\"result\",\"total_cost_usd\":0.5,\"usage\":{\"input_tokens\":3,\"output_tokens\":4}}'";
        let mut h = SessionHandle::spawn(&sh(script)).expect("spawn");
        let events = h.collect_events().await;
        h.shutdown().await;
        use regatta_core::stream::NormalizedEvent::*;
        assert_eq!(events.len(), 3, "the garbage line must be skipped");
        assert!(matches!(events[0], SessionStarted { .. }));
        assert!(matches!(events[1], AssistantText { .. }));
        assert!(matches!(events[2], Usage { .. }));
    }

    #[tokio::test]
    async fn skips_unparseable_lines() {
        let mut h =
            SessionHandle::spawn(&sh("echo 'not json'; echo '{}'; echo ''")).expect("spawn");
        let events = h.collect_events().await;
        h.shutdown().await;
        assert!(events.is_empty(), "garbage stdout yields no events");
    }

    #[test]
    fn tail_transcript_reads_incrementally_from_offset() {
        use std::io::Write;
        let path = std::env::temp_dir().join(format!("regatta_tx_{}.jsonl", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let l1 = "{\"type\":\"system\",\"subtype\":\"init\",\"model\":\"m\"}\n";
        let l2 = "{\"type\":\"assistant\",\"message\":{\"content\":[{\"type\":\"text\",\"text\":\"hi\"}]}}\n";
        std::fs::write(&path, format!("{l1}{l2}")).unwrap();

        let (events, off) = tail_transcript(&path, 0).unwrap();
        assert_eq!(events.len(), 2);

        let l3 = "{\"type\":\"result\",\"total_cost_usd\":0.5,\"usage\":{\"input_tokens\":1,\"output_tokens\":2}}\n";
        let partial = "{\"type\":\"assistant\""; // no newline — must be held for next time
        let mut f = std::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .unwrap();
        write!(f, "{l3}{partial}").unwrap();
        drop(f);

        let (events2, off2) = tail_transcript(&path, off).unwrap();
        assert_eq!(
            events2.len(),
            1,
            "only the completed line; the partial is held"
        );
        assert!(off2 > off);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn tail_transcript_restarts_after_truncation() {
        let path =
            std::env::temp_dir().join(format!("regatta_tx_trunc_{}.jsonl", std::process::id()));
        std::fs::write(
            &path,
            "{\"type\":\"system\",\"subtype\":\"init\",\"model\":\"m\"}\n",
        )
        .unwrap();
        let (events, _) = tail_transcript(&path, 9_999_999).unwrap(); // offset past EOF
        assert_eq!(events.len(), 1);
        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn autopauses_when_budget_exceeded_and_block() {
        use regatta_core::budget::{Budget, BudgetAction};
        let budget = Budget {
            limit_usd: 10.0,
            action: BudgetAction::Block,
        };
        let mut h = SessionHandle::spawn(&sh("sleep 30")).expect("spawn");
        let pid = h.pid().unwrap() as i32;
        // under budget → no pause; still alive
        assert!(!h.autopause_if_exceeded(5.0, &budget).await);
        assert!(alive(pid));
        // over budget + Block → auto-pause (teardown)
        assert!(h.autopause_if_exceeded(12.0, &budget).await);
        tokio::time::sleep(Duration::from_millis(250)).await;
        assert!(
            !alive(pid),
            "exceeding the ceiling must tear the session down"
        );
    }

    #[tokio::test]
    async fn does_not_autopause_when_action_is_not_block() {
        use regatta_core::budget::{Budget, BudgetAction};
        let budget = Budget {
            limit_usd: 10.0,
            action: BudgetAction::Warn,
        };
        let mut h = SessionHandle::spawn(&sh("sleep 30")).expect("spawn");
        let pid = h.pid().unwrap() as i32;
        // way over budget, but the action is Warn → no pause
        assert!(!h.autopause_if_exceeded(50.0, &budget).await);
        assert!(alive(pid));
        h.shutdown().await;
    }

    #[tokio::test]
    async fn codex_runs_through_the_same_pipeline_as_claude() {
        use regatta_core::stream::NormalizedEvent::*;
        // a fake Codex backend emitting real Codex JSON shapes on stdout
        let codex = "echo '{\"type\":\"event_msg\",\"payload\":{\"type\":\"task_started\",\"model\":\"gpt-5-codex\"}}'; \
                     echo '{\"type\":\"response_item\",\"payload\":{\"type\":\"message\",\"role\":\"assistant\",\"content\":[{\"type\":\"output_text\",\"text\":\"hi\"}]}}'; \
                     echo '{\"type\":\"event_msg\",\"payload\":{\"type\":\"token_count\",\"info\":{\"total_token_usage\":{\"input_tokens\":5,\"output_tokens\":2}}}}'";
        let mut h = SessionHandle::spawn(&sh(codex)).expect("spawn");
        let pid = h.pid().unwrap() as i32;
        let events = h.collect_events_with(Backend::Codex).await;
        h.shutdown().await;
        // Codex produces the SAME normalized shape as Claude — SessionStarted, AssistantText, Usage —
        // through the same spawn/collect/teardown path, with zero view-layer special-casing.
        assert_eq!(events.len(), 3);
        assert!(matches!(events[0], SessionStarted { .. }));
        assert!(matches!(events[1], AssistantText { .. }));
        assert!(matches!(events[2], Usage { .. }));
        tokio::time::sleep(Duration::from_millis(150)).await;
        assert!(
            !alive(pid),
            "the zero-leak teardown reaps the Codex process too"
        );
    }

    #[tokio::test]
    async fn pump_feeds_the_registry_live() {
        use regatta_core::runtime::Registry;
        // a fake session emitting Codex JSON; pump each event into the live registry as it arrives
        let codex = "echo '{\"type\":\"event_msg\",\"payload\":{\"type\":\"task_started\",\"model\":\"gpt-5-codex\"}}'; \
                     echo '{\"type\":\"response_item\",\"payload\":{\"type\":\"message\",\"role\":\"assistant\",\"content\":[{\"type\":\"output_text\",\"text\":\"hi\"}]}}'";
        let mut h = SessionHandle::spawn(&sh(codex)).expect("spawn");
        let mut reg = Registry::default();
        h.pump_events_with(Backend::Codex, |ev| reg.apply("live-1", &ev))
            .await;
        h.shutdown().await;
        let rt = reg.get("live-1").expect("session is in the registry");
        assert_eq!(rt.model, "gpt-5-codex");
        assert_eq!(rt.last_text, "hi");
        assert_eq!(rt.turns, 1);
    }

    #[tokio::test]
    async fn claude_session_runs_live_through_the_pipeline() {
        use regatta_core::runtime::Registry;
        // a fake `claude -p` emitting real Claude stream-json: init → assistant → result
        let claude = "echo '{\"type\":\"system\",\"subtype\":\"init\",\"model\":\"claude-opus-4-8\"}'; \
                      echo '{\"type\":\"assistant\",\"message\":{\"content\":[{\"type\":\"text\",\"text\":\"editing oauth.ts\"}]}}'; \
                      echo '{\"type\":\"result\",\"total_cost_usd\":0.37,\"usage\":{\"input_tokens\":1200,\"output_tokens\":340}}'";
        let mut h = SessionHandle::spawn(&sh(claude)).expect("spawn");
        let pid = h.pid().unwrap() as i32;
        let mut reg = Registry::default();
        h.pump_events_with(Backend::Claude, |ev| reg.apply("live-claude", &ev))
            .await;
        h.shutdown().await;
        let rt = reg.get("live-claude").expect("session in the registry");
        assert_eq!(rt.model, "claude-opus-4-8");
        assert_eq!(rt.last_text, "editing oauth.ts");
        assert!((rt.cost_usd - 0.37).abs() < 1e-9); // authoritative total_cost_usd
        assert_eq!(rt.turns, 1);
        tokio::time::sleep(Duration::from_millis(150)).await;
        assert!(!alive(pid), "the lived session is reaped on teardown");
    }
}
