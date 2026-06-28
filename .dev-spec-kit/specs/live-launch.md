# Feature: Live launch + pump

> User story: As Regatta, I want a spawned session's events pumped into the live registry as they
> arrive, so the cockpit reflects the running process in real time — the heart of Go Live.
> Intake: M7 — wire the supervisor to the live registry.

## Requirement R-LIVE-LAUNCH-01 — pump a live session into the registry

WHEN a spawned session's events are pumped THEN the system SHALL apply each `NormalizedEvent` to the live `Registry` as it arrives, so the registry reflects the running session (model, last text, turns, cost).

@check kind=integration ref=regatta_supervisor::tests::pump_feeds_the_registry_live
