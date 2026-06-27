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
}
