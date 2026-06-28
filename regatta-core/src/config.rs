//! Pure layered configuration: global → project → session, with later layers overriding earlier.

use std::collections::BTreeMap;

/// One configuration layer: a set of key→value settings.
pub type ConfigLayer = BTreeMap<String, String>;

/// Resolve layers into one effective config — later layers override earlier keys.
pub fn resolve(layers: &[ConfigLayer]) -> ConfigLayer {
    let mut out = ConfigLayer::new();
    for layer in layers {
        for (k, v) in layer {
            out.insert(k.clone(), v.clone());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn later_layers_override_earlier() {
        let global = ConfigLayer::from([
            ("model".to_string(), "haiku".to_string()),
            ("theme".to_string(), "dark".to_string()),
        ]);
        let project = ConfigLayer::from([
            ("model".to_string(), "sonnet".to_string()),
            ("dir".to_string(), "/repo".to_string()),
        ]);
        let session = ConfigLayer::from([("model".to_string(), "opus".to_string())]);
        let eff = resolve(&[global, project, session]);
        assert_eq!(eff.get("model").map(String::as_str), Some("opus")); // session wins
        assert_eq!(eff.get("theme").map(String::as_str), Some("dark")); // inherited from global
        assert_eq!(eff.get("dir").map(String::as_str), Some("/repo")); // inherited from project
    }

    #[test]
    fn empty_layers_resolve_empty() {
        assert!(resolve(&[]).is_empty());
    }
}
