# Feature: Burn rate & prediction

> User story: As Regatta, I want to know how fast a session is spending and when it will hit its
> budget, so I can warn early and auto-pause before a runaway burns money.
> Intake: M3 — drives the "exceeds in ~N min" warning + auto-pause timing.

## Requirement R-BURN-01 — burn rate and time-to-ceiling

WHEN computing spend rate THEN the system SHALL return USD/hour from total spent and elapsed seconds (zero elapsed → zero rate).

@check kind=unit ref=regatta_core::cost::tests::computes_burn_rate

WHEN predicting THEN the system SHALL return the seconds until spend reaches the limit at the current rate — None if spend is not advancing, and zero if already at or over the limit.

@check kind=unit ref=regatta_core::cost::tests::predicts_time_to_ceiling
