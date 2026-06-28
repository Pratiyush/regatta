//! Parse Claude transcript (`~/.claude/projects/<slug>/<id>.jsonl`) session metadata, so the
//! Resume board can index every past session from one line — never loading the whole file.

use crate::slug::slugify;
use serde_json::Value;

/// Session metadata extracted from a transcript line.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct SessionMeta {
    pub session_id: String,
    pub cwd: String,
    pub project: String,
    pub git_branch: String,
}

/// Parse session metadata from a transcript line. Returns None for non-JSON or id-less lines.
pub fn parse_session_meta(line: &str) -> Option<SessionMeta> {
    let v: Value = serde_json::from_str(line).ok()?;
    let session_id = v.get("sessionId")?.as_str()?.to_string();
    let cwd = v
        .get("cwd")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let git_branch = v
        .get("gitBranch")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let project = slugify(cwd.rsplit('/').next().unwrap_or_default());
    Some(SessionMeta {
        session_id,
        cwd,
        project,
        git_branch,
    })
}

/// Parse a Codex rollout's `session_meta` line into the SAME `SessionMeta` as a Claude transcript, so
/// Codex rollouts appear in the Resume board alongside Claude sessions.
pub fn parse_codex_meta(line: &str) -> Option<SessionMeta> {
    let v: Value = serde_json::from_str(line).ok()?;
    if v.get("type")?.as_str()? != "session_meta" {
        return None;
    }
    let payload = v.get("payload")?;
    let session_id = payload.get("id")?.as_str()?.to_string();
    let cwd = payload
        .get("cwd")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let project = slugify(cwd.rsplit('/').next().unwrap_or_default());
    Some(SessionMeta {
        session_id,
        cwd,
        project,
        git_branch: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_session_meta() {
        let line = r#"{"sessionId":"abc-123","cwd":"/Users/p/Desktop/AI/terminal-x","gitBranch":"main","type":"user"}"#;
        assert_eq!(
            parse_session_meta(line),
            Some(SessionMeta {
                session_id: "abc-123".into(),
                cwd: "/Users/p/Desktop/AI/terminal-x".into(),
                project: "terminal-x".into(),
                git_branch: "main".into(),
            })
        );
        // Minimal line: only the session id — defaults stay safe, project falls back via slugify.
        assert_eq!(
            parse_session_meta(r#"{"sessionId":"x"}"#),
            Some(SessionMeta {
                session_id: "x".into(),
                cwd: String::new(),
                project: "session".into(),
                git_branch: String::new(),
            })
        );
    }

    #[test]
    fn rejects_malformed_or_idless() {
        assert_eq!(parse_session_meta("not json"), None);
        assert_eq!(parse_session_meta("{}"), None);
        assert_eq!(parse_session_meta(r#"{"sessionId":5}"#), None); // id not a string
    }

    #[test]
    fn parses_codex_session_meta() {
        let line = r#"{"type":"session_meta","payload":{"id":"cx-abc","cwd":"/Users/p/proj","originator":"cli"}}"#;
        assert_eq!(
            parse_codex_meta(line),
            Some(SessionMeta {
                session_id: "cx-abc".into(),
                cwd: "/Users/p/proj".into(),
                project: "proj".into(),
                git_branch: String::new(),
            })
        );
        assert_eq!(
            parse_codex_meta(r#"{"type":"session_meta","payload":{"id":"x"}}"#),
            Some(SessionMeta {
                session_id: "x".into(),
                cwd: String::new(),
                project: "session".into(),
                git_branch: String::new(),
            })
        );
    }

    #[test]
    fn rejects_non_codex_meta() {
        assert_eq!(parse_codex_meta("not json"), None);
        assert_eq!(parse_codex_meta("{}"), None); // no type
        assert_eq!(parse_codex_meta(r#"{"type":5}"#), None); // type not a string
        assert_eq!(
            parse_codex_meta(r#"{"type":"event_msg","payload":{}}"#),
            None
        ); // not session_meta
        assert_eq!(parse_codex_meta(r#"{"type":"session_meta"}"#), None); // no payload
        assert_eq!(
            parse_codex_meta(r#"{"type":"session_meta","payload":{}}"#),
            None
        ); // no id
        assert_eq!(
            parse_codex_meta(r#"{"type":"session_meta","payload":{"id":5}}"#),
            None
        ); // id not a string
    }
}
