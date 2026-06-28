//! Build the launch plan for an agent backend. Pure: produces argv/env/cwd; spawning is glue.

use std::path::PathBuf;

/// A fully-resolved plan for launching (or resuming) an agent process. No I/O — the supervisor runs it.
#[derive(Debug, Clone, PartialEq)]
pub struct LaunchPlan {
    pub program: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub cwd: PathBuf,
}

/// Plan a headless Claude Code session in stream-json duplex mode.
/// When `resume` is true, attach to the existing `session_id`; otherwise force a new one.
pub fn plan_claude_launch(model: &str, session_id: &str, cwd: &str, resume: bool) -> LaunchPlan {
    let mut args = vec![
        "-p".to_string(),
        "--output-format".into(),
        "stream-json".into(),
        "--input-format".into(),
        "stream-json".into(),
        "--include-partial-messages".into(),
        "--model".into(),
        model.into(),
        "--permission-prompt-tool".into(),
        "mcp__regatta__approve".into(),
    ];
    if resume {
        args.push("--resume".into());
        args.push(session_id.into());
    } else {
        args.push("--session-id".into());
        args.push(session_id.into());
    }
    LaunchPlan {
        program: "claude".into(),
        args,
        env: vec![("REGATTA_SESSION_ID".into(), session_id.into())],
        cwd: PathBuf::from(cwd),
    }
}

/// Plan a launch that resumes an existing session (attaches via `--resume`).
pub fn plan_resume(model: &str, session_id: &str, cwd: &str) -> LaunchPlan {
    plan_claude_launch(model, session_id, cwd, true)
}

/// The shell command to resume a session in a terminal — the Resume board's Copy button.
pub fn resume_command(session_id: &str) -> String {
    format!("claude --resume {session_id}")
}

/// Return a copy of `plan` with `extra` env merged in — extra overrides existing keys, new keys
/// appended. This is how the materialized layered config reaches every session's launch.
pub fn with_env(mut plan: LaunchPlan, extra: &[(String, String)]) -> LaunchPlan {
    for (k, v) in extra {
        if let Some(slot) = plan.env.iter_mut().find(|(ek, _)| ek == k) {
            slot.1 = v.clone();
        } else {
            plan.env.push((k.clone(), v.clone()));
        }
    }
    plan
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> LaunchPlan {
        plan_claude_launch("claude-opus-4-8", "sid-1", "/repo", false)
    }

    #[test]
    fn plans_a_fresh_session() {
        let p = fresh();
        assert_eq!(p.program, "claude");
        assert_eq!(p.cwd, PathBuf::from("/repo"));
        for flag in [
            "-p",
            "--output-format",
            "stream-json",
            "--include-partial-messages",
        ] {
            assert!(p.args.iter().any(|a| a == flag), "missing {flag}");
        }
        assert!(p
            .args
            .windows(2)
            .any(|w| w == ["--model", "claude-opus-4-8"]));
        assert!(p.args.windows(2).any(|w| w == ["--session-id", "sid-1"]));
        assert!(p.args.iter().any(|a| a == "--permission-prompt-tool"));
        assert!(!p.args.iter().any(|a| a == "--resume"));
    }

    #[test]
    fn plans_a_resume() {
        let p = plan_claude_launch("claude-opus-4-8", "sid-9", "/repo", true);
        assert!(p.args.windows(2).any(|w| w == ["--resume", "sid-9"]));
        assert!(!p.args.iter().any(|a| a == "--session-id"));
    }

    #[test]
    fn tags_env_for_reaping() {
        let p = fresh();
        assert!(p
            .env
            .iter()
            .any(|(k, v)| k == "REGATTA_SESSION_ID" && v == "sid-1"));
    }

    #[test]
    fn plans_a_resume_and_command() {
        let p = plan_resume("claude-opus-4-8", "sid-7", "/repo");
        assert!(p.args.windows(2).any(|w| w == ["--resume", "sid-7"]));
        assert_eq!(resume_command("sid-7"), "claude --resume sid-7");
    }

    #[test]
    fn resume_command_handles_empty_id() {
        assert_eq!(resume_command(""), "claude --resume ");
    }

    #[test]
    fn with_env_merges_overriding_existing() {
        let base = plan_claude_launch("opus", "s1", "/r", false); // env has REGATTA_SESSION_ID=s1
        let merged = with_env(
            base,
            &[
                ("REGATTA_SESSION_ID".to_string(), "override".to_string()),
                (
                    "ANTHROPIC_BASE_URL".to_string(),
                    "http://localhost".to_string(),
                ),
            ],
        );
        assert!(merged
            .env
            .iter()
            .any(|(k, v)| k == "REGATTA_SESSION_ID" && v == "override")); // overridden
        assert!(merged
            .env
            .iter()
            .any(|(k, v)| k == "ANTHROPIC_BASE_URL" && v == "http://localhost")); // appended
        assert_eq!(
            merged
                .env
                .iter()
                .filter(|(k, _)| k == "REGATTA_SESSION_ID")
                .count(),
            1
        ); // no duplicate
    }

    #[test]
    fn with_env_empty_is_unchanged() {
        let base = plan_claude_launch("opus", "s1", "/r", false);
        let n = base.env.len();
        assert_eq!(with_env(base, &[]).env.len(), n);
    }
}
