# Feature: Live dock line

> User story: As Regatta, I want each live session to render a compact dock line, so the cockpit shows
> a real running session at a glance.
> Intake: M7 — the live wiring's view shape.

## Requirement R-LIVEVIEW-01 — summarize a live session

WHEN summarizing a `SessionRuntime` THEN the system SHALL produce a compact line "model · N turns · $X.XX", using an em dash for an unknown model.

@check kind=unit ref=regatta_core::runtime::tests::summary_is_a_compact_dock_line
