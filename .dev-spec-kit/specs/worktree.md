# Feature: Worktree-per-session plan

> User story: As Regatta, I want each session to get its own git worktree on a unique branch, so two
> agents on the same repo never collide and cleanup is trivial.
> Intake: M1 — pure planning; the glue runs `git worktree add/remove` from this plan.

## Requirement R-WORKTREE-01 — plan an isolated worktree

WHEN planning a worktree for a session THEN the system SHALL produce a unique branch `regatta/<slug>-<short-uuid>`, an isolated checkout path under the worktrees directory, and the base branch — so sessions on the same repo never collide.

@check kind=unit ref=regatta_core::worktree::tests::plans_a_unique_worktree

IF the uuid is shorter than 8 characters THEN the system SHALL use the whole uuid and never panic.

@check kind=unit ref=regatta_core::worktree::tests::handles_a_short_uuid
