# Feature: Codex stream parser

> User story: As Regatta, I want Codex's events normalized into the SAME shape as Claude's, so every
> view (grid, board, usage, teardown) works for Codex with zero special-casing — the multi-backend proof.
> Intake: M6 — the second backend.

## Requirement R-CODEX-01 — parse Codex events into the normalized shape

WHEN parsing a Codex rollout / `codex exec --json` line THEN the system SHALL map `task_started` → SessionStarted, `token_count` → Usage, and an assistant `message` → AssistantText — the SAME `NormalizedEvent` Claude produces.

@check kind=unit ref=regatta_core::stream::tests::parses_codex_events_into_normalized

IF the line is metadata (session_meta / turn_context), a non-assistant or unknown event, or malformed THEN the system SHALL return None.

@check kind=unit ref=regatta_core::stream::tests::codex_skips_meta_and_unknown
