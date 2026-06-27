# Feature: Agent launch plan

> User story: As the supervisor, I want a pure, fully-resolved plan for launching (or resuming) an
> agent process, so spawning is trivial glue and the plan itself is unit-tested.
> Intake: M1 — the first piece of the session supervisor; the AgentBackend abstraction begins here.

## Requirement R-LAUNCH-01 — plan a Claude Code session

WHEN planning a fresh Claude session THEN the plan SHALL run `claude` in headless stream-json duplex mode with the chosen model, a forced `--session-id`, the Regatta approval tool (`--permission-prompt-tool`), and a `REGATTA_SESSION_ID` environment tag.

@check kind=unit ref=regatta_core::backend::tests::plans_a_fresh_session

IF resuming an existing session THEN the plan SHALL use `--resume <id>` in place of `--session-id <id>`.

@check kind=unit ref=regatta_core::backend::tests::plans_a_resume

## Security

The plan SHALL tag every agent process with a `REGATTA_SESSION_ID` environment variable so the supervisor can identify and reap stray processes — no leaked agents, ever.

@check kind=unit ref=regatta_core::backend::tests::tags_env_for_reaping
