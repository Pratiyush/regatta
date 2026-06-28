//! The MCP permission-tool response contract for inline approvals — the JSON `mcp__regatta__approve`
//! returns to the agent: allow echoes the input back as `updatedInput`; deny carries a `message`.

use serde_json::{json, Value};

/// An approval decision from the dock.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Decision {
    Allow,
    Deny,
}

/// Build the JSON the approve tool returns. Allow echoes `original_input` as `updatedInput` (null if
/// the input is malformed); Deny carries `reason` as `message`.
pub fn approval_response(decision: Decision, original_input: &str, reason: &str) -> String {
    match decision {
        Decision::Allow => {
            let input: Value = serde_json::from_str(original_input).unwrap_or(Value::Null);
            json!({ "behavior": "allow", "updatedInput": input }).to_string()
        }
        Decision::Deny => json!({ "behavior": "deny", "message": reason }).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allow_echoes_input_as_updated() {
        assert_eq!(
            approval_response(Decision::Allow, r#"{"command":"ls"}"#, ""),
            r#"{"behavior":"allow","updatedInput":{"command":"ls"}}"#
        );
        // malformed input → null updatedInput, never panics
        assert_eq!(
            approval_response(Decision::Allow, "not json", ""),
            r#"{"behavior":"allow","updatedInput":null}"#
        );
    }

    #[test]
    fn deny_carries_the_reason() {
        assert_eq!(
            approval_response(Decision::Deny, r#"{"command":"rm -rf /"}"#, "too risky"),
            r#"{"behavior":"deny","message":"too risky"}"#
        );
        assert_ne!(Decision::Allow, Decision::Deny); // exercise the derived equality
    }

    #[test]
    fn allow_round_trip_drives_the_agent() {
        use crate::stream::{parse_approval_request, NormalizedEvent};
        // the agent's approve request → the event the dock surfaces
        let event =
            parse_approval_request(r#"{"tool_name":"Bash","input":{"command":"cargo test"}}"#)
                .expect("a parsed approval request");
        assert_eq!(
            event,
            NormalizedEvent::ApprovalRequested {
                tool: "Bash".into(),
                detail: "cargo test".into()
            }
        );
        // the dock allows → the JSON the agent receives back (echoes the input → it proceeds)
        assert_eq!(
            approval_response(Decision::Allow, r#"{"command":"cargo test"}"#, ""),
            r#"{"behavior":"allow","updatedInput":{"command":"cargo test"}}"#
        );
    }

    #[test]
    fn deny_round_trip_stops_the_agent() {
        use crate::stream::{parse_approval_request, NormalizedEvent};
        let event =
            parse_approval_request(r#"{"tool_name":"Bash","input":{"command":"rm -rf /"}}"#)
                .expect("a parsed approval request");
        assert!(matches!(event, NormalizedEvent::ApprovalRequested { .. }));
        // the dock denies → the agent receives a deny with the reason (it does not proceed)
        assert_eq!(
            approval_response(
                Decision::Deny,
                r#"{"command":"rm -rf /"}"#,
                "destructive — blocked"
            ),
            r#"{"behavior":"deny","message":"destructive — blocked"}"#
        );
    }
}
