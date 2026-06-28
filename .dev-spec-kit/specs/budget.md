# Feature: Budget model

> User story: As Regatta, I want to classify a session's spend against its budget and decide when to
> auto-pause, so a runaway agent stops at the ceiling instead of burning money unattended.
> Intake: M3 — drives the budget warning + auto-pause.

## Requirement R-BUDGET-01 — classify spend and decide auto-pause

WHEN classifying spend against a budget THEN the system SHALL return Ok, Warn (at ≥80% of the limit), or Exceeded (at ≥100%); a non-positive limit is unbounded (Ok).

@check kind=unit ref=regatta_core::budget::tests::classifies_spend_against_budget

WHEN deciding auto-pause THEN the system SHALL pause only when spend is Exceeded and the budget's action is Block.

@check kind=unit ref=regatta_core::budget::tests::pauses_only_when_exceeded_and_block
