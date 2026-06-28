//! Normalize Claude Code's `--output-format stream-json` lines into backend-agnostic events.

use serde_json::Value;

/// A normalized event every view can consume, regardless of which backend produced it.
#[derive(Debug, Clone, PartialEq)]
pub enum NormalizedEvent {
    SessionStarted {
        model: String,
    },
    AssistantText {
        text: String,
    },
    Usage {
        cost_usd: f64,
        input: u64,
        output: u64,
        cache_read: u64,
        cache_create: u64,
    },
}

/// Parse one newline-delimited stream-json line. Returns `None` for blank, unknown, or malformed lines.
pub fn parse_claude_line(line: &str) -> Option<NormalizedEvent> {
    let v: Value = serde_json::from_str(line).ok()?;
    match v.get("type")?.as_str()? {
        "system" if v.get("subtype").and_then(Value::as_str) == Some("init") => {
            Some(NormalizedEvent::SessionStarted {
                model: v
                    .get("model")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
            })
        }
        "assistant" => Some(NormalizedEvent::AssistantText {
            text: assistant_text(&v),
        }),
        "result" => {
            let usage = v.get("usage");
            Some(NormalizedEvent::Usage {
                cost_usd: v
                    .get("total_cost_usd")
                    .and_then(Value::as_f64)
                    .unwrap_or(0.0),
                input: usage_field(usage, "input_tokens"),
                output: usage_field(usage, "output_tokens"),
                cache_read: usage_field(usage, "cache_read_input_tokens"),
                cache_create: usage_field(usage, "cache_creation_input_tokens"),
            })
        }
        _ => None,
    }
}

fn usage_field(usage: Option<&Value>, key: &str) -> u64 {
    usage
        .and_then(|u| u.get(key))
        .and_then(Value::as_u64)
        .unwrap_or(0)
}

/// Concatenate the text of an assistant message's `text` content blocks (skipping tool_use, etc.).
fn assistant_text(v: &Value) -> String {
    v.get("message")
        .and_then(|m| m.get("content"))
        .and_then(Value::as_array)
        .map(|blocks| {
            blocks
                .iter()
                .filter(|b| b.get("type").and_then(Value::as_str) == Some("text"))
                .filter_map(|b| b.get("text").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default()
}

/// Parse one Codex rollout / `codex exec --json` line into the SAME `NormalizedEvent` as Claude — the
/// proof the backend abstraction holds. Returns None for metadata, unknown, or malformed lines.
pub fn parse_codex_line(line: &str) -> Option<NormalizedEvent> {
    let v: Value = serde_json::from_str(line).ok()?;
    let payload = v.get("payload")?;
    match v.get("type")?.as_str()? {
        "event_msg" => match payload.get("type")?.as_str()? {
            "task_started" => Some(NormalizedEvent::SessionStarted {
                model: payload
                    .get("model")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
            }),
            "token_count" => {
                let u = payload.get("info").and_then(|i| i.get("total_token_usage"));
                Some(NormalizedEvent::Usage {
                    cost_usd: 0.0,
                    input: codex_usage(u, "input_tokens"),
                    output: codex_usage(u, "output_tokens"),
                    cache_read: codex_usage(u, "cached_input_tokens"),
                    cache_create: 0,
                })
            }
            _ => None,
        },
        "response_item"
            if payload.get("type").and_then(Value::as_str) == Some("message")
                && payload.get("role").and_then(Value::as_str) == Some("assistant") =>
        {
            Some(NormalizedEvent::AssistantText {
                text: codex_text(payload),
            })
        }
        _ => None,
    }
}

fn codex_usage(usage: Option<&Value>, key: &str) -> u64 {
    usage
        .and_then(|u| u.get(key))
        .and_then(Value::as_u64)
        .unwrap_or(0)
}

fn codex_text(payload: &Value) -> String {
    payload
        .get("content")
        .and_then(Value::as_array)
        .map(|parts| {
            parts
                .iter()
                .filter_map(|p| p.get("text").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_lines() {
        assert_eq!(
            parse_claude_line(r#"{"type":"system","subtype":"init","model":"claude-opus-4-8"}"#),
            Some(NormalizedEvent::SessionStarted {
                model: "claude-opus-4-8".into()
            })
        );
        assert_eq!(
            parse_claude_line(r#"{"type":"system","subtype":"init"}"#),
            Some(NormalizedEvent::SessionStarted {
                model: String::new()
            })
        );
        assert_eq!(
            parse_claude_line(
                r#"{"type":"assistant","message":{"content":[{"type":"tool_use"},{"type":"text","text":"Hel"},{"type":"text","text":"lo"}]}}"#
            ),
            Some(NormalizedEvent::AssistantText {
                text: "Hello".into()
            })
        );
        assert_eq!(
            parse_claude_line(r#"{"type":"assistant"}"#),
            Some(NormalizedEvent::AssistantText {
                text: String::new()
            })
        );
        assert_eq!(
            parse_claude_line(
                r#"{"type":"result","total_cost_usd":0.42,"usage":{"input_tokens":10,"output_tokens":20,"cache_read_input_tokens":5,"cache_creation_input_tokens":3}}"#
            ),
            Some(NormalizedEvent::Usage {
                cost_usd: 0.42,
                input: 10,
                output: 20,
                cache_read: 5,
                cache_create: 3
            })
        );
        assert_eq!(
            parse_claude_line(r#"{"type":"result"}"#),
            Some(NormalizedEvent::Usage {
                cost_usd: 0.0,
                input: 0,
                output: 0,
                cache_read: 0,
                cache_create: 0
            })
        );
    }

    #[test]
    fn ignores_malformed_and_unknown() {
        assert_eq!(parse_claude_line("not json"), None);
        assert_eq!(parse_claude_line("{}"), None);
        assert_eq!(parse_claude_line(r#"{"type":5}"#), None);
        assert_eq!(parse_claude_line(r#"{"type":"user"}"#), None);
        assert_eq!(
            parse_claude_line(r#"{"type":"system","subtype":"status"}"#),
            None
        );
    }

    #[test]
    fn parses_codex_events_into_normalized() {
        assert_eq!(
            parse_codex_line(
                r#"{"type":"event_msg","payload":{"type":"task_started","model":"gpt-5-codex"}}"#
            ),
            Some(NormalizedEvent::SessionStarted {
                model: "gpt-5-codex".into()
            })
        );
        assert_eq!(
            parse_codex_line(
                r#"{"type":"response_item","payload":{"type":"message","role":"assistant","content":[{"type":"output_text","text":"Hel"},{"type":"output_text","text":"lo"}]}}"#
            ),
            Some(NormalizedEvent::AssistantText {
                text: "Hello".into()
            })
        );
        assert_eq!(
            parse_codex_line(
                r#"{"type":"event_msg","payload":{"type":"token_count","info":{"total_token_usage":{"input_tokens":10883,"output_tokens":79,"cached_input_tokens":5}}}}"#
            ),
            Some(NormalizedEvent::Usage {
                cost_usd: 0.0,
                input: 10883,
                output: 79,
                cache_read: 5,
                cache_create: 0
            })
        );
        assert_eq!(
            parse_codex_line(r#"{"type":"event_msg","payload":{"type":"token_count"}}"#),
            Some(NormalizedEvent::Usage {
                cost_usd: 0.0,
                input: 0,
                output: 0,
                cache_read: 0,
                cache_create: 0
            })
        );
        assert_eq!(
            parse_codex_line(
                r#"{"type":"response_item","payload":{"type":"message","role":"assistant"}}"#
            ),
            Some(NormalizedEvent::AssistantText {
                text: String::new()
            })
        );
    }

    #[test]
    fn codex_skips_meta_and_unknown() {
        assert_eq!(parse_codex_line("not json"), None); // malformed
        assert_eq!(parse_codex_line("{}"), None); // no payload
        assert_eq!(parse_codex_line(r#"{"payload":{}}"#), None); // no type
        assert_eq!(parse_codex_line(r#"{"type":5,"payload":{}}"#), None); // type not a string
        assert_eq!(
            parse_codex_line(r#"{"type":"event_msg","payload":{}}"#),
            None
        ); // no payload.type
        assert_eq!(
            parse_codex_line(r#"{"type":"session_meta","payload":{"id":"x"}}"#),
            None
        ); // meta
        assert_eq!(
            parse_codex_line(r#"{"type":"turn_context","payload":{}}"#),
            None
        );
        assert_eq!(
            parse_codex_line(r#"{"type":"event_msg","payload":{"type":"task_complete"}}"#),
            None
        );
        assert_eq!(
            parse_codex_line(
                r#"{"type":"response_item","payload":{"type":"message","role":"user"}}"#
            ),
            None
        ); // not assistant
        assert_eq!(
            parse_codex_line(r#"{"type":"response_item","payload":{"type":"reasoning"}}"#),
            None
        ); // a response_item that isn't a message
        assert_eq!(
            parse_codex_line(r#"{"type":"event_msg","payload":{"type":5}}"#),
            None
        ); // event_msg whose payload.type isn't a string
    }
}
