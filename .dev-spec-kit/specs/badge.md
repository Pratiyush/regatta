# Feature: Backend badge

> User story: As Regatta, I want every session to show whether it's Claude or Codex, and the launcher
> to pick a backend, so the multi-backend world is visible and selectable.
> Intake: M6 — the backend badge + selector.

## Requirement R-BADGE-01 — parse a backend label

WHEN parsing a backend label THEN the system SHALL map "Claude" / "Codex" to a `Backend` (round-tripping with `label()`); an unknown label returns None.

@check kind=unit ref=regatta_core::backend::tests::backend_from_label_roundtrips
