# Feature: Cost rollups store

> User story: As Regatta, I want to persist priced usage events and roll them up by total, project, and
> time window, so the Usage view shows live spend and budgets can be enforced.
> Intake: M3 — the durable cost ledger behind the Usage view + budgets.

## Requirement R-COSTSTORE-01 — record and roll up cost

WHEN cost events are recorded THEN `total_spend`, `spend_by_project` (most-spent first), and `spend_since(ts)` SHALL reflect them.

@check kind=integration ref=regatta_store::tests::records_and_rolls_up_cost

WHEN no cost events exist THEN the rollups SHALL be zero/empty rather than error.

@check kind=integration ref=regatta_store::tests::empty_cost_rollups_are_zero
