//! M7 payload-executing adversarial review — the live reducer/registry fold parser-derived events
//! into long-lived state. These prove no overflow/panic under hostile magnitudes + scale.

use regatta_core::runtime::Registry;
use regatta_core::stream::NormalizedEvent::*;

#[test]
fn runtime_survives_token_overflow() {
    let mut reg = Registry::default();
    // repeated max-u64 token usage must saturate, never overflow/panic
    for _ in 0..5 {
        reg.apply(
            "s",
            &Usage {
                cost_usd: 0.0,
                input: u64::MAX,
                output: u64::MAX,
                cache_read: 0,
                cache_create: 0,
            },
        );
    }
    let rt = reg.get("s").unwrap();
    assert_eq!(rt.input_tokens, u64::MAX); // saturated, not wrapped
    assert_eq!(rt.output_tokens, u64::MAX);
}

#[test]
fn registry_survives_scale_and_huge_text() {
    let mut reg = Registry::default();
    let big = "x".repeat(100_000);
    for i in 0..5_000 {
        reg.apply(&format!("s{i}"), &SessionStarted { model: "m".into() });
        reg.apply(&format!("s{i}"), &AssistantText { text: big.clone() });
    }
    assert_eq!(reg.len(), 5_000);
    let snap = reg.snapshot(); // a large snapshot must not panic
    assert_eq!(snap.len(), 5_000);
    // summaries over the whole set stay total
    assert!(snap.iter().all(|(_, rt)| !rt.summary().is_empty()));
}
