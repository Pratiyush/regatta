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

/// Whether a key names a secret whose value must be masked.
pub fn is_secret_key(key: &str) -> bool {
    let k = key.to_ascii_uppercase();
    ["KEY", "TOKEN", "SECRET", "PASSWORD", "PASSWD"]
        .iter()
        .any(|n| k.contains(n))
}

/// Mask a secret value: keep the last 4 chars, bullet the rest (values ≤ 4 chars fully masked).
pub fn mask(value: &str) -> String {
    let n = value.chars().count();
    if n <= 4 {
        "•".repeat(n)
    } else {
        let tail: String = value.chars().skip(n - 4).collect();
        format!("{}{}", "•".repeat(n - 4), tail)
    }
}

/// A copy of `config` with secret values masked — for display and logs.
pub fn masked(config: &ConfigLayer) -> ConfigLayer {
    config
        .iter()
        .map(|(k, v)| {
            let val = if is_secret_key(k) { mask(v) } else { v.clone() };
            (k.clone(), val)
        })
        .collect()
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

    #[test]
    fn flags_secret_keys() {
        assert!(is_secret_key("ANTHROPIC_API_KEY"));
        assert!(is_secret_key("GITHUB_TOKEN"));
        assert!(is_secret_key("my_secret"));
        assert!(is_secret_key("DB_PASSWORD"));
        assert!(!is_secret_key("model"));
        assert!(!is_secret_key("working_dir"));
    }

    #[test]
    fn masks_secret_values() {
        let m = mask("sk-ant-1234abcd"); // 15 chars
        assert!(m.ends_with("abcd"));
        assert_eq!(m.chars().count(), 15);
        assert_eq!(m.chars().filter(|&c| c == '•').count(), 11);
        assert_eq!(mask("abcd"), "••••"); // ≤4 fully masked
        assert_eq!(mask(""), ""); // empty
        let cfg = ConfigLayer::from([
            (
                "ANTHROPIC_API_KEY".to_string(),
                "sk-ant-1234abcd".to_string(),
            ),
            ("model".to_string(), "opus".to_string()),
        ]);
        let v = masked(&cfg);
        assert!(v.get("ANTHROPIC_API_KEY").unwrap().ends_with("abcd"));
        assert_ne!(
            v.get("ANTHROPIC_API_KEY").map(String::as_str),
            Some("sk-ant-1234abcd")
        );
        assert_eq!(v.get("model").map(String::as_str), Some("opus")); // non-secret untouched
    }
}
