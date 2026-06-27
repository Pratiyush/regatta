//! Payload-executing adversarial checks for the pure core's untrusted-input surfaces
//! (M1 close security review). These run hostile inputs through the real functions and assert
//! they stay safe — no panic, no injection, no traversal.

use regatta_core::slug::slugify;
use regatta_core::stream::parse_claude_line;
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
