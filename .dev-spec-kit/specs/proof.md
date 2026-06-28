# Feature: Multi-backend indistinguishability

> User story: As Regatta, I want a Codex session to run through the exact same supervisor pipeline as
> Claude — spawn, collect normalized events, zero-leak teardown — so the whole app is backend-agnostic.
> Intake: M6 — the DoD proof that the M1 abstraction held.

## Requirement R-PROOF-01 — Codex is indistinguishable through the pipeline

WHEN a Codex session runs through the supervisor THEN it SHALL produce the same `NormalizedEvent` shape (SessionStarted / AssistantText / Usage) as Claude, and the zero-leak teardown SHALL reap it.

@check kind=integration ref=regatta_supervisor::tests::codex_runs_through_the_same_pipeline_as_claude
