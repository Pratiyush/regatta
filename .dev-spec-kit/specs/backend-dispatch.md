# Feature: Backend dispatch

> User story: As Regatta, I want one `Backend` type that routes launch + parsing to Claude or Codex,
> so every glue/UI path is backend-agnostic — the proof the M1 abstraction held.
> Intake: M6 — the multi-backend dispatch point.

## Requirement R-BACKEND-01 — dispatch by backend

WHEN dispatching by backend THEN `label` and `plan_launch` SHALL route to the right backend (Claude vs Codex).

@check kind=unit ref=regatta_core::backend::tests::backend_dispatches_label_and_launch

WHEN parsing a line THEN each backend SHALL parse its own format and return None for the other's.

@check kind=unit ref=regatta_core::backend::tests::backend_parses_its_own_format
