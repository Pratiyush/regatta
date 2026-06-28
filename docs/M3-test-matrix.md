# M3 — Cost & Budget Governance: 7-type Test Matrix

Recorded at the M3 close (2026-06-28). M3 made spend legible and bounded: pricing, rollups, burn-rate
prediction, budgets with auto-pause, and the live Usage view.

| # | Type | M3 coverage | Status |
|---|------|-------------|--------|
| 1 | **Unit 100%** | `regatta_core` (cost: `price_tokens`/`effective_cost`/`burn_rate`/`time_to_ceiling`/`budget_pct`; budget: `budget_status`/`should_pause`) at 100% line+branch (749 regions). | ✅ |
| 2 | **Edge/boundary** | budget thresholds at exactly 80% / 100%; zero elapsed → zero burn; non-positive limit unbounded; empty rollups → 0 (`COALESCE`); cache read/write multipliers. | ✅ |
| 3 | **Invalid/untrusted** | `cost_and_budget_math_never_panics_on_extremes` (u64::MAX tokens; f64 MAX/MIN/∞/NaN spends; zero/negative limits) — no panic, no divide-by-zero; `budget_pct` stays 0..100. | ✅ |
| 4 | **Scenario/integration** | `cost_events` ledger + rollups (total / by-project / by-model / since); **auto-pause kills a real over-budget process**, spares an under-budget and a non-Block one. | ✅ |
| 5 | **E2e-smoke** | app builds + launches; the Usage view renders live aggregates; auto-pause is wired to the zero-leak teardown. | ✅ |
| 6 | **Performance/scale** | rollups are single indexed SQL aggregates (`idx_cost_ts`); pricing + budget math is O(1). | ✅ |
| 7 | **UI/UX·a11y** | render-and-look of the Usage view: spend/burn cards, color-coded budget bar (green/amber/red by status), by-project + by-model bars; clickable spend pill. | ✅ |

Artifact: `scratchpad/m3-usage.png`.
