# Feature: Usage view

> User story: As Regatta, I want to see live spend — total, by project, by model, against budget — so I
> know where the money goes and how close I am to a ceiling.
> Intake: M3 — the Usage view's data + budget bar.

## Requirement R-USAGE-01 — budget percent and spend by model

WHEN rendering the budget bar THEN `budget_pct` SHALL return the percent of the budget consumed, clamped to 0..100 (a non-positive limit reads as 0%).

@check kind=unit ref=regatta_core::cost::tests::budget_pct_clamps_0_to_100

WHEN showing the model split THEN `spend_by_model` SHALL group spend by model, most-spent first.

@check kind=integration ref=regatta_store::tests::rolls_up_spend_by_model
