# Feature: Live session runtime

> User story: As Regatta, I want a pure reducer that folds a session's event stream into the state the
> cockpit reflects, so the live UI is a deterministic function of the normalized events — the spine of
> Go Live (M7).
> Intake: M7 — wire real streaming sessions into the live UI.

## Requirement R-RUNTIME-01 — fold the event stream into live state

WHEN folding a `NormalizedEvent` into a `SessionRuntime` THEN the system SHALL set the model on SessionStarted, set the last text + increment the turn count on AssistantText, and accumulate tokens + cost on Usage.

@check kind=unit ref=regatta_core::runtime::tests::folds_events_into_live_state

WHEN a Usage event carries no authoritative `cost_usd` THEN its cost SHALL be priced from the tokens by model (Codex/local), via `cost::effective_cost`.

@check kind=unit ref=regatta_core::runtime::tests::prices_usage_by_model_when_no_authoritative_usd
