# Feature: Apply config to a session launch

> User story: As Regatta, I want a session's launch to carry the materialized config env, so a setting
> toggled once is actually applied to every new session ("one system, one team").
> Intake: M5 — config → session launch.

## Requirement R-APPLY-01 — merge config env into a launch plan

WHEN applying config env to a launch plan THEN the system SHALL merge the extra env — overriding existing keys and appending new ones, with no duplicates.

@check kind=unit ref=regatta_core::backend::tests::with_env_merges_overriding_existing

WHEN the extra env is empty THEN the launch plan SHALL be unchanged.

@check kind=unit ref=regatta_core::backend::tests::with_env_empty_is_unchanged
