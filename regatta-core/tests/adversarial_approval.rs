//! M8 payload-executing adversarial review — the approval surface ingests an agent's tool-use request
//! (untrusted) and emits the response the agent receives. These prove no panic on hostile input, and
//! that shell metacharacters / huge payloads are inert DATA — never executed, never able to break the
//! response JSON.

use regatta_core::approval::{approval_response, Decision};
use regatta_core::stream::parse_approval_request;

#[test]
fn approval_parse_survives_hostile_input() {
    let big = "A".repeat(1_000_000);
    let payloads: Vec<String> = vec![
        String::new(),
        "\0\0".into(),
        "[]".into(),
        "{".repeat(400), // serde recursion-limits, no stack overflow
        format!(r#"{{"tool_name":"Bash","input":{{"command":"{big}"}}}}"#), // 1 MB command
        r#"{"tool_name":"Bash; rm -rf / #","input":{"command":"$(whoami)`id`"}}"#.into(), // injection-y strings
        r#"{"tool_name":"X","input":{"command":["array","not","string"]}}"#.into(), // command type confusion
        r#"{"tool_name":"X","input":42}"#.into(), // input not an object
        "\u{1d4ca}\u{1d4c3}\u{1d4be}\u{1d4ec}\u{1f480}\u{202e}".into(), // math-script + skull + RTL override
    ];
    for p in &payloads {
        let _ = parse_approval_request(p); // the absence of a panic is the proof
    }
}

#[test]
fn approval_response_is_always_valid_json() {
    let huge = "x".repeat(500_000);
    // a hostile original_input must never break the JSON or panic
    for input in ["not json", "", "\0", "{unbalanced", huge.as_str()] {
        let allow = approval_response(Decision::Allow, input, "");
        assert!(allow.starts_with(r#"{"behavior":"allow""#));
        let _: serde_json::Value = serde_json::from_str(&allow).expect("allow is valid JSON");
    }
    // a reason with quotes / newlines / shell metacharacters is JSON-escaped — inert
    let deny = approval_response(Decision::Deny, "{}", "he said \"rm -rf\"\n; drop table");
    let parsed: serde_json::Value = serde_json::from_str(&deny).expect("deny is valid JSON");
    assert_eq!(parsed["behavior"], "deny");
    assert_eq!(parsed["message"], "he said \"rm -rf\"\n; drop table");
}
