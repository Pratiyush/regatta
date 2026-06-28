# M3 — Cost & Budget Governance: Adversarial Security Review

Recorded at the M3 close (2026-06-28). M3 added arithmetic on untrusted/config values (token counts,
costs, budgets) and a new SQL ledger. This review *executes* extreme payloads against the real math.

## New attack surface
- `regatta_core::cost` — `price_tokens`, `effective_cost`, `burn_rate`, `time_to_ceiling`, `budget_pct`.
- `regatta_core::budget` — `budget_status`, `should_pause`.
- `regatta_store` — `cost_events` INSERT + the spend rollup queries.
- Tauri `usage_view` command + the auto-pause teardown.

## Findings
**None requiring a fix** — the math is robust by construction. (Contrast M2, where the review found
and fixed two hardenings.)

## Verified safe (executed payloads)
- **No panic / overflow / divide-by-zero.** `cost_and_budget_math_never_panics_on_extremes` runs
  `price_tokens` with u64::MAX token counts, and `budget_pct` / `burn_rate` / `time_to_ceiling` /
  `budget_status` with f64 MAX / MIN / ∞ / NaN spends and zero/negative limits. All return; `budget_pct`
  always stays in 0..100 (an explicit clamp before a saturating float→int cast); every division is
  guarded (zero elapsed → 0; rate ≤ 0 → None; non-positive limit → unbounded / 0%).
- **No SQL injection in the ledger.** `record_cost` binds all fields via `?` parameters; the rollup
  queries are static with no interpolated input. (The transcript board search was proven in M2.)
- **Auto-pause is fail-safe.** It pauses *only* on Exceeded **and** Block, reusing the proven zero-leak
  group teardown; a corrupt or extreme spend can at worst trigger a (safe) teardown, never an invalid
  state.

## Standing properties
- The pure core stays I/O-free and deterministic; glue is thin and integration-tested. `cargo deny`
  green. **M3 added no third-party dependencies.**
