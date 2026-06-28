//! Map normalized events to human-readable lines for the cockpit's Focus stream.

use crate::stream::NormalizedEvent;

/// A display line for the session stream: a short role label + its text.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct EventLine {
    pub role: String,
    pub text: String,
}

/// Turn a normalized event into a human-readable display line.
pub fn event_line(ev: &NormalizedEvent) -> EventLine {
    match ev {
        NormalizedEvent::SessionStarted { model } => EventLine {
            role: "system".into(),
            text: format!("session started · {model}"),
        },
        NormalizedEvent::AssistantText { text } => EventLine {
            role: "claude".into(),
            text: text.clone(),
        },
        NormalizedEvent::Usage {
            cost_usd,
            input,
            output,
            ..
        } => EventLine {
            role: "usage".into(),
            text: format!("${cost_usd:.2} · {input} in / {output} out"),
        },
        NormalizedEvent::ApprovalRequested { tool, detail } => EventLine {
            role: "approval".into(),
            text: format!("⚠ approval needed · {tool}: {detail}"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_each_event_kind() {
        assert_eq!(
            event_line(&NormalizedEvent::SessionStarted {
                model: "opus".into()
            }),
            EventLine {
                role: "system".into(),
                text: "session started · opus".into()
            }
        );
        assert_eq!(
            event_line(&NormalizedEvent::AssistantText {
                text: "hello".into()
            }),
            EventLine {
                role: "claude".into(),
                text: "hello".into()
            }
        );
        assert_eq!(
            event_line(&NormalizedEvent::Usage {
                cost_usd: 0.5,
                input: 3,
                output: 4,
                cache_read: 0,
                cache_create: 0
            }),
            EventLine {
                role: "usage".into(),
                text: "$0.50 · 3 in / 4 out".into()
            }
        );
        assert_eq!(
            event_line(&NormalizedEvent::ApprovalRequested {
                tool: "Bash".into(),
                detail: "rm -rf /tmp/x".into()
            }),
            EventLine {
                role: "approval".into(),
                text: "⚠ approval needed · Bash: rm -rf /tmp/x".into()
            }
        );
    }

    #[test]
    fn preserves_empty_assistant_text() {
        assert_eq!(
            event_line(&NormalizedEvent::AssistantText {
                text: String::new()
            }),
            EventLine {
                role: "claude".into(),
                text: String::new()
            }
        );
    }
}
