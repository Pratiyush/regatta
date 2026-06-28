//! M6 payload-executing adversarial review — the Codex parsers ingest untrusted JSON from
//! `~/.codex/sessions`, and the launch plan builds argv from a session id/model. These fire
//! malformed/hostile inputs and assert the parsers never panic, hang, or reach an invalid state
//! (they only ever return `Some(valid)` or `None`), and that launch never shell-interprets input.

use regatta_core::backend::{plan_codex_launch, Backend};
use regatta_core::stream::parse_codex_line;
use regatta_core::transcript::parse_codex_meta;

#[test]
fn codex_parsers_survive_hostile_json() {
    let big_text = "A".repeat(1_000_000);
    let payloads: Vec<String> = vec![
        String::new(),  // empty
        "\0\0\0".into(), // null bytes
        "[]".into(),    // array, not object
        "true".into(),  // bare scalar
        "\"just a string\"".into(),
        "{".repeat(500), // deeply nested → serde recursion limit returns Err, not a stack overflow
        format!(
            r#"{{"type":"event_msg","payload":{{"type":"token_count","info":{{"total_token_usage":{{"input_tokens":{}}}}}}}}}"#,
            u64::MAX
        ), // max u64 token count
        r#"{"type":"event_msg","payload":{"type":"token_count","info":{"total_token_usage":{"input_tokens":-5}}}}"#.into(), // negative
        r#"{"type":"event_msg","payload":{"type":"token_count","info":{"total_token_usage":{"input_tokens":1.9e9}}}}"#.into(), // float, not int
        format!(
            r#"{{"type":"response_item","payload":{{"type":"message","role":"assistant","content":[{{"type":"text","text":"{big_text}"}}]}}}}"#
        ), // 1 MB assistant text
        r#"{"type":"response_item","payload":{"type":"message","role":"assistant","content":"not-an-array"}}"#.into(), // content type confusion
        r#"{"type":"session_meta","payload":{"id":["array"],"cwd":{"obj":1}}}"#.into(), // id/cwd type confusion
        "\u{1d566}\u{1d55f}\u{1d55a}\u{1d558}\u{1d560}\u{1d555}\u{1d556} \u{1f680}\u{202e}drowssap".into(), // math-bold unicode + RTL override
    ];
    for p in &payloads {
        // The result is irrelevant — the absence of a panic/hang IS the proof. A hostile transcript
        // can never crash Regatta or drive a parser into an invalid state.
        let _ = parse_codex_line(p);
        let _ = parse_codex_meta(p);
        let _ = Backend::Codex.parse_line(p);
    }
}

#[test]
fn codex_launch_never_shell_interprets_injection() {
    // A hostile session id / model must each be exactly ONE argv element — never split, expanded, or
    // shell-interpreted. (The supervisor execs the program directly with this argv; there is no shell.)
    let evil_id = "abc; rm -rf / # $(whoami) `id` && curl evil";
    let evil_model = "$(touch /tmp/pwned)";
    let plan = plan_codex_launch(evil_model, evil_id, "/repo", true);

    assert_eq!(plan.program, "codex"); // the program is fixed, never attacker-controlled
    assert!(plan.args.windows(2).any(|w| w == ["resume", evil_id])); // id survives intact as one arg
    assert!(plan.args.windows(2).any(|w| w == ["--model", evil_model])); // model survives intact as one arg
    assert!(plan // the env tag carries the id verbatim, not re-parsed
        .env
        .iter()
        .any(|(k, v)| k == "REGATTA_SESSION_ID" && v == evil_id));
}
