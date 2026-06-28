# Feature: Cost math

> User story: As Regatta, I want to know what each session costs, so I can show live spend and enforce
> budgets — using the agent's authoritative USD when it reports one, and pricing tokens when it doesn't.
> Intake: M3 — the foundation of cost governance.

## Requirement R-COST-01 — price token usage and pick the effective cost

WHEN pricing token usage THEN the system SHALL apply per-model rates (Opus 4.8 $5/$25, Sonnet 4.6 $3/$15, Haiku 4.5 $1/$5 per 1M tokens; cache reads at 0.1× the input rate, cache writes at 1.25×; unknown/local models cost $0).

@check kind=unit ref=regatta_core::cost::tests::prices_tokens_by_model

WHEN computing a usage event's cost THEN the system SHALL prefer the authoritative `cost_usd` if present, otherwise price the tokens; a non-usage event costs nothing.

@check kind=unit ref=regatta_core::cost::tests::effective_cost_prefers_authoritative_usd
