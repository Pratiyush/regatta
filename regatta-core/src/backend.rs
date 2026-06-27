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
}
