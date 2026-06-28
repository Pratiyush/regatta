# Feature: Codex launch plan

> User story: As Regatta, I want to launch a Codex session the same way I launch Claude — a pure
> argv/env/cwd plan the supervisor runs — so the spawn/teardown path is shared.
> Intake: M6 — the second backend's launch.

## Requirement R-CODEXLAUNCH-01 — plan a Codex launch

WHEN planning a Codex launch THEN the system SHALL produce a `LaunchPlan` with program `codex`, args `exec --json --model <model>`, and the `REGATTA_SESSION_ID` env tag (for reaping).

@check kind=unit ref=regatta_core::backend::tests::plans_a_codex_session

IF `resume` is true THEN the plan SHALL attach to the rollout via `resume <session_id>`.

@check kind=unit ref=regatta_core::backend::tests::plans_a_codex_resume
