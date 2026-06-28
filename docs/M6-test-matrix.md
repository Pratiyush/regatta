# M6 — Codex Backend: 7-type test matrix

Recorded at the M6 close (the final milestone). M6 makes Codex a second concrete backend behind the
same `Backend` abstraction; the bar is that Codex and Claude are **indistinguishable** through the
whole pipeline — parse, launch, dispatch, board, teardown — with zero view-layer special-casing.

| # | Type | M6 evidence | Status |
|---|------|-------------|--------|
| 1 | **Unit 100%** | `regatta_core` lines 100% (CI `--fail-under-lines 100`): `parse_codex_line`, `plan_codex_launch`, `Backend::{label,plan_launch,parse_line,from_label}`, `parse_codex_meta` | ✅ |
| 2 | **Edge/boundary** | `adversarial_codex::codex_parsers_survive_hostile_json` — empty / null-bytes / array / scalar / 500-deep nesting / max-u64 / negative / float / 1 MB text / type-confusion / unicode+RTL → no panic | ✅ |
| 3 | **Invalid/untrusted** | the same battery fired at `parse_codex_line` + `parse_codex_meta` + `Backend::Codex.parse_line`; `codex_launch_never_shell_interprets_injection` proves argv isolation | ✅ |
| 4 | **Scenario/integration** | `codex_runs_through_the_same_pipeline_as_claude` — a real Codex-JSON process is spawned, collected via `collect_events_with(Backend::Codex)`, yields the same `NormalizedEvent` shape, and the teardown reaps it | ✅ |
| 5 | **E2e-smoke** | the badge UI built (release frontend + debug binary) + render-and-look (`scratchpad/m6-badge.png`): Codex + Claude sessions side-by-side in one grid/dock | ✅ |
| 6 | **Performance/scale** | a 1 MB assistant-text payload parses without quadratic blowup (text concat is linear); the supervisor proof reaps the Codex process — the zero-leak guarantee holds for the 2nd backend | ✅ |
| 7 | **UI/UX·a11y** | the backend badge is **dot+label+shape**, not color-only — a pill with the text "CLAUDE"/"CODEX" (orange/green); render-and-look confirms legibility in both the project rail and the dock | ✅ |

**DoD met:** a Codex and a Claude session run in the same grid/board/usage — parsed, launched, resumed
(board), and torn down — with **zero view-layer special-casing**. The M1 `AgentBackend` abstraction held:
the only view-layer change for a whole second backend was a label on a badge.
