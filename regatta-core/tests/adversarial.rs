//! Payload-executing adversarial checks for the pure core's untrusted-input surfaces
//! (M1 close security review). These run hostile inputs through the real functions and assert
//! they stay safe — no panic, no injection, no traversal.

use regatta_core::board::recency_group;
use regatta_core::budget::{budget_status, should_pause, Budget, BudgetAction};
use regatta_core::cost::{budget_pct, burn_rate, effective_cost, price_tokens, time_to_ceiling};
use regatta_core::slug::slugify;
use regatta_core::stream::parse_claude_line;
use regatta_core::transcript::parse_session_meta;
use regatta_core::worktree::plan_worktree;

/// Session names flow into git branch names and filesystem paths — they must be neutralized.
#[test]
fn slugify_neutralizes_shell_and_path_payloads() {
    let big = "x".repeat(10_000);
    let payloads = [
        "; rm -rf /",
        "../../etc/passwd",
        "$(reboot)",
        "a`whoami`b",
        "..\\..\\windows",
        "name\nwith\nnewlines\ttabs",
        "--force",
        "🔥💥 emoji 漢字",
        big.as_str(),
        "",
    ];
    for evil in payloads {
        let s = slugify(evil);
        assert!(
            s.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
            "slug from {evil:?} contains unsafe chars: {s:?}"
        );
        assert!(
            !s.is_empty(),
            "slug must never be empty (got from {evil:?})"
        );
        assert!(
            !s.starts_with('-') && !s.ends_with('-'),
            "stray hyphen: {s:?}"
        );
    }
}

/// Agent stdout is fully untrusted — the parser must never panic, however hostile the bytes.
#[test]
fn parser_never_panics_on_hostile_input() {
    let many_blocks = format!(
        "{{\"type\":\"assistant\",\"message\":{{\"content\":[{}]}}}}",
        "{\"type\":\"text\",\"text\":\"a\"},".repeat(20_000)
    );
    let payloads = vec![
        String::new(),
        "\0\0\0 null bytes".to_string(),
        "{".to_string(),
        "[1,2,3]".to_string(),
        "\"just a string\"".to_string(),
        "{\"type\":123}".to_string(),
        "x".repeat(2_000_000), // 2 MB single line
        many_blocks,
        // numeric overflow / extreme values must degrade to defaults, not panic
        "{\"type\":\"result\",\"total_cost_usd\":1e308,\"usage\":{\"input_tokens\":99999999999999999999999}}".to_string(),
    ];
    for p in &payloads {
        let _ = parse_claude_line(p); // the assertion is simply: this returns, never panics
    }
}

/// A hostile session name must not escape the worktree branch namespace or the checkout directory.
#[test]
fn worktree_plan_resists_traversal_and_injection() {
    let p = plan_worktree(
        "repo",
        "../../.. ; rm -rf / #",
        "main",
        "deadbeefcafe",
        "/wt",
    );
    let suffix = p
        .branch
        .strip_prefix("regatta/")
        .expect("branch is namespaced");
    assert!(
        suffix
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
        "branch suffix unsafe: {suffix:?}"
    );
    assert!(
        !p.path.contains(".."),
        "worktree path allows traversal: {:?}",
        p.path
    );
    assert!(
        p.path.starts_with("/wt/"),
        "worktree escaped its directory: {:?}",
        p.path
    );
}

/// Transcript lines are read from disk and may be corrupt or hostile — parsing them for the Resume
/// index must never panic, and a traversal-laden cwd must not produce a traversal-laden project.
#[test]
fn transcript_meta_never_panics_on_hostile_input() {
    let deep = format!("{}1{}", "[".repeat(5_000), "]".repeat(5_000)); // deep nesting
    let traversal = format!(
        "{{\"sessionId\":\"x\",\"cwd\":\"{}\"}}",
        "../".repeat(5_000)
    );
    let payloads = vec![
        String::new(),
        "not json".to_string(),
        "{}".to_string(),
        "{\"sessionId\":123}".to_string(), // wrong type
        "{\"sessionId\":[1,2]}".to_string(),
        "{\"sessionId\":\"x\",\"cwd\":123}".to_string(),
        "{\"sessionId\":\"\\u0000\\u0000\"}".to_string(), // null bytes
        format!("{{\"sessionId\":\"{}\"}}", "x".repeat(2_000_000)), // 2 MB id
        traversal,
        deep,
    ];
    for p in &payloads {
        if let Some(meta) = parse_session_meta(p) {
            assert!(
                !meta.project.contains(".."),
                "project label allows traversal: {:?}",
                meta.project
            );
        }
    }
}

/// last_activity comes from a file mtime or a stored integer and could be extreme or corrupt;
/// recency bucketing must not overflow on i64 extremes.
#[test]
fn recency_group_never_overflows() {
    let extremes = [i64::MIN, i64::MAX, 0, -1, i64::MAX / 2, i64::MIN / 2];
    for &la in &extremes {
        for &now in &extremes {
            let _ = recency_group(la, now); // must return a bucket, never panic
        }
    }
}

/// Token counts, costs, and budgets come from disk/config and could be extreme or corrupt — the cost
/// and budget math must never panic, divide by zero, or overflow into an out-of-range percent.
#[test]
fn cost_and_budget_math_never_panics_on_extremes() {
    let big = u64::MAX;
    for model in [
        "claude-opus-4-8",
        "claude-sonnet-4-6",
        "claude-haiku-4-5",
        "local",
        "",
    ] {
        let _ = price_tokens(model, big, big, big, big);
    }
    for spent in [0.0, -1.0, f64::MAX, f64::MIN, f64::INFINITY, f64::NAN] {
        for limit in [0.0, -5.0, 1e-12, f64::MAX, f64::INFINITY] {
            assert!(budget_pct(spent, limit) <= 100, "budget_pct out of range");
        }
    }
    let _ = burn_rate(f64::MAX, 0);
    let _ = burn_rate(f64::NAN, big);
    let _ = time_to_ceiling(f64::MAX, f64::MIN_POSITIVE, f64::MAX);
    let _ = time_to_ceiling(f64::NAN, f64::NAN, f64::NAN);
    for action in [
        BudgetAction::Warn,
        BudgetAction::Throttle,
        BudgetAction::Block,
    ] {
        let b = Budget {
            limit_usd: 0.0,
            action,
        };
        let _ = should_pause(budget_status(f64::MAX, &b), action);
    }
    let _ = effective_cost(
        "opus",
        &regatta_core::stream::NormalizedEvent::Usage {
            cost_usd: 0.0,
            input: big,
            output: big,
            cache_read: big,
            cache_create: big,
        },
    );
}
