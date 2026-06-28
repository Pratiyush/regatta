# Feature: Codex session meta

> User story: As Regatta, I want Codex rollouts indexed into the same Resume board as Claude sessions,
> so I resume either backend from one place — the multi-backend board.
> Intake: M6 — Codex in the board.

## Requirement R-CODEXMETA-01 — parse a Codex session_meta line

WHEN parsing a Codex rollout's `session_meta` line THEN the system SHALL extract the id and cwd into the SAME `SessionMeta` a Claude transcript produces (id, cwd, project label, empty git branch).

@check kind=unit ref=regatta_core::transcript::tests::parses_codex_session_meta

IF the line is not a `session_meta`, is malformed, or has no id THEN the system SHALL return None.

@check kind=unit ref=regatta_core::transcript::tests::rejects_non_codex_meta
