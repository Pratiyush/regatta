//! The live session runtime — folds a session's normalized event stream into the state every view
//! reflects (model, last assistant text, turn count, accumulated cost + tokens). Pure + deterministic.

use crate::stream::NormalizedEvent;
use std::collections::BTreeMap;

/// The live, view-facing state of one session, accumulated from its `NormalizedEvent` stream.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SessionRuntime {
    pub model: String,
    pub last_text: String,
    pub turns: u64,
    pub cost_usd: f64,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

impl SessionRuntime {
    /// Fold one normalized event into the live state. Usage cost uses the authoritative `cost_usd`
    /// when present (Claude), else prices the tokens by model (Codex/local) via `cost::effective_cost`.
    pub fn apply_event(&mut self, event: &NormalizedEvent) {
        match event {
            NormalizedEvent::SessionStarted { model } => self.model = model.clone(),
            NormalizedEvent::AssistantText { text } => {
                self.last_text = text.clone();
                self.turns += 1;
            }
            NormalizedEvent::Usage { input, output, .. } => {
                self.input_tokens = self.input_tokens.saturating_add(*input);
                self.output_tokens = self.output_tokens.saturating_add(*output);
                self.cost_usd += crate::cost::effective_cost(&self.model, event);
            }
        }
    }
}

/// All live sessions keyed by id — the single source the cockpit renders.
#[derive(Debug, Clone, Default)]
pub struct Registry {
    sessions: BTreeMap<String, SessionRuntime>,
}

impl Registry {
    /// Fold an event into the session's runtime, creating the session on first sight.
    pub fn apply(&mut self, id: &str, event: &NormalizedEvent) {
        self.sessions
            .entry(id.to_string())
            .or_default()
            .apply_event(event);
    }

    /// The live runtime for `id`, if the session exists.
    pub fn get(&self, id: &str) -> Option<&SessionRuntime> {
        self.sessions.get(id)
    }

    /// How many live sessions are tracked.
    pub fn len(&self) -> usize {
        self.sessions.len()
    }

    /// Whether no sessions are tracked yet.
    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    /// A stable, id-sorted snapshot of every session for the UI.
    pub fn snapshot(&self) -> Vec<(String, SessionRuntime)> {
        self.sessions
            .iter()
            .map(|(id, rt)| (id.clone(), rt.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_events_into_live_state() {
        let mut rt = SessionRuntime::default();
        rt.apply_event(&NormalizedEvent::SessionStarted {
            model: "claude-opus-4-8".into(),
        });
        rt.apply_event(&NormalizedEvent::AssistantText {
            text: "first".into(),
        });
        rt.apply_event(&NormalizedEvent::AssistantText {
            text: "second".into(),
        });
        rt.apply_event(&NormalizedEvent::Usage {
            cost_usd: 0.42, // authoritative (Claude)
            input: 10,
            output: 20,
            cache_read: 0,
            cache_create: 0,
        });
        assert_eq!(
            rt,
            SessionRuntime {
                model: "claude-opus-4-8".into(),
                last_text: "second".into(),
                turns: 2,
                cost_usd: 0.42,
                input_tokens: 10,
                output_tokens: 20,
            }
        );
    }

    #[test]
    fn prices_usage_by_model_when_no_authoritative_usd() {
        let mut rt = SessionRuntime {
            model: "claude-opus-4-8".into(),
            ..Default::default()
        };
        // no cost_usd → priced via the table (Opus $5 / 1M input)
        rt.apply_event(&NormalizedEvent::Usage {
            cost_usd: 0.0,
            input: 1_000_000,
            output: 0,
            cache_read: 0,
            cache_create: 0,
        });
        assert!((rt.cost_usd - 5.0).abs() < 1e-9);
        assert_eq!(rt.input_tokens, 1_000_000);
    }

    #[test]
    fn registry_tracks_sessions_by_id() {
        let mut reg = Registry::default();
        assert!(reg.is_empty());
        reg.apply(
            "s1",
            &NormalizedEvent::SessionStarted {
                model: "claude-opus-4-8".into(),
            },
        );
        reg.apply("s1", &NormalizedEvent::AssistantText { text: "hi".into() });
        reg.apply(
            "s2",
            &NormalizedEvent::SessionStarted {
                model: "gpt-5-codex".into(),
            },
        );
        assert_eq!(reg.len(), 2);
        assert!(!reg.is_empty());
        assert_eq!(reg.get("s1").unwrap().turns, 1);
        assert_eq!(reg.get("s1").unwrap().model, "claude-opus-4-8");
        assert_eq!(reg.get("s2").unwrap().model, "gpt-5-codex");
        assert!(reg.get("nope").is_none());
    }

    #[test]
    fn snapshot_is_id_sorted() {
        let mut reg = Registry::default();
        reg.apply(
            "zebra",
            &NormalizedEvent::AssistantText { text: "z".into() },
        );
        reg.apply(
            "alpha",
            &NormalizedEvent::AssistantText { text: "a".into() },
        );
        let snap = reg.snapshot();
        let ids: Vec<&str> = snap.iter().map(|(id, _)| id.as_str()).collect();
        assert_eq!(ids, ["alpha", "zebra"]); // BTreeMap → id-sorted
        assert_eq!(snap[0].1.last_text, "a");
    }
}
