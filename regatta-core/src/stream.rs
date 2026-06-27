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
}
