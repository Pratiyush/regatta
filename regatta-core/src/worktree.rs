//! Plan a per-session git worktree: a unique branch and an isolated checkout path.
//! Pure planning only — the glue runs `git worktree add/remove` from this plan.

use crate::slug::slugify;

/// A resolved plan for a session's isolated worktree. No I/O.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct WorktreePlan {
    pub branch: String,
    pub path: String,
    pub base: String,
}

/// Plan a worktree for `session_name` on `repo`, branched from `base`. `uuid` makes the branch and
/// path unique (so two sessions on the same repo never collide); `worktrees_dir` is where checkouts live.
pub fn plan_worktree(
    repo: &str,
    session_name: &str,
    base: &str,
    uuid: &str,
    worktrees_dir: &str,
) -> WorktreePlan {
    let slug = slugify(session_name);
    let short = &uuid[..uuid.len().min(8)];
    WorktreePlan {
        branch: format!("regatta/{slug}-{short}"),
        path: format!("{worktrees_dir}/{repo}-{slug}-{short}"),
        base: base.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plans_a_unique_worktree() {
        let p = plan_worktree(
            "payments-svc",
            "Fix: idempotency!",
            "main",
            "abcd1234efgh",
            "/wt",
        );
        assert_eq!(
            p,
            WorktreePlan {
                branch: "regatta/fix-idempotency-abcd1234".into(),
                path: "/wt/payments-svc-fix-idempotency-abcd1234".into(),
                base: "main".into(),
            }
        );
    }

    #[test]
    fn handles_a_short_uuid() {
        let p = plan_worktree("r", "x", "main", "ab", "/wt");
        assert_eq!(p.branch, "regatta/x-ab");
        assert_eq!(p.path, "/wt/r-x-ab");
    }
}
