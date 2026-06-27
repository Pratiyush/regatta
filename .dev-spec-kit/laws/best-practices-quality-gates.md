---
inclusion: always
---
# Best practices — quality gates (every slice & release)

## Per slice (the loop)
- EARS spec in `.dev-spec-kit/specs/`; every criterion bound by `@check`.
- TDD: write the failing test → `rivet check run … --expect-red` → implement → `rivet check run …` green.
- `rivet task done` (the gate) → `rivet drift` → `rivet graph build` → `rivet gate --quiet` (the ONLY green check — never grep graph text).
- 100% `helm_core` coverage on touched code · `clippy -D warnings` clean · `rustfmt` · a screenshot/story for any UI.
- Conventional-Commit PR **authored by the human (never AI-co-authored)**; `rivet guard pr` must pass.

## Per release (milestone close)
- Record the full **7-type test matrix**: unit-100% · edge/boundary · invalid/untrusted · scenario/integration · e2e-smoke · perf-with-budget · UI·a11y.
- A **payload-executing adversarial review** before the tag.
- CHANGELOG + docs updated; tag cut by `release-please` (never hand-bump/hand-tag).
- **Render-it-and-look**; **refine-before** + **gap-after** the milestone.
