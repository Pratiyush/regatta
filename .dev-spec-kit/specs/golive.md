# Feature: Go-live lived test

> User story: As Regatta, I want a real Claude session to run live through the whole launch→pump→registry
> path (as Codex already does), so both backends are proven live end-to-end — the M7 DoD.
> Intake: M7 — the lived test.

## Requirement R-GOLIVE-01 — a Claude session runs live through the pipeline

WHEN a Claude session is spawned and pumped THEN the system SHALL reflect it live in the registry (model, last text, authoritative cost, turns) and the teardown SHALL reap it — proving both backends run live through one path.

@check kind=integration ref=regatta_supervisor::tests::claude_session_runs_live_through_the_pipeline
