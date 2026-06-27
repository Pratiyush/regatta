//! Glue: spawn an agent process in its own process group, with guaranteed teardown of the whole tree.
//!
//! This is Regatta's reliability promise: when a session ends we kill the agent **and every child it
//! spawned**, leaving nothing behind — the failure mode that leaves cmux helpers spinning for days.

use regatta_core::backend::LaunchPlan;
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

    /// Read the agent's stdout to EOF, parsing each line into a normalized event and skipping
    /// unparseable lines.
    pub async fn collect_events(&mut self) -> Vec<regatta_core::stream::NormalizedEvent> {
        use tokio::io::{AsyncBufReadExt, BufReader};
        let mut events = Vec::new();
        if let Some(stdout) = self.child.stdout.take() {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if let Some(ev) = regatta_core::stream::parse_claude_line(&line) {
                    events.push(ev);
                }
            }
        }
        events
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
}
