# M7 — Go Live: 7-type test matrix

Recorded at the M7 close. M7 wires real spawned sessions into the live cockpit: a pure reducer +
registry fold the event stream into view state; the supervisor pumps live; Tauri serves + renders it.

| # | Type | M7 evidence | Status |
|---|------|-------------|--------|
| 1 | **Unit 100%** | `regatta_core` lines 100% (CI): `SessionRuntime::{apply_event,summary}`, `Registry::{apply,get,snapshot}` | ✅ |
| 2 | **Edge/boundary** | `adversarial_runtime::runtime_survives_token_overflow` — repeated `u64::MAX` usage saturates (no wrap) | ✅ |
| 3 | **Invalid/untrusted** | events flow from the M6-fuzzed parsers; `launch_session` argv is `plan_launch` (injection-contained, proven M6) | ✅ |
| 4 | **Scenario/integration** | `pump_feeds_the_registry_live` (Codex) + `claude_session_runs_live_through_the_pipeline` (Claude) — **both** backends spawn→pump→registry→teardown | ✅ |
| 5 | **E2e-smoke** | frontend + src-tauri build; the **Live tab renders** in the cockpit | ⚠️ live-content screenshot blocked by an unreachable macOS Space; the tab is confirmed |
| 6 | **Performance/scale** | `registry_survives_scale_and_huge_text` — 5,000 sessions + 100 KB texts; snapshot stays total | ✅ |
| 7 | **UI/UX·a11y** | Live cards carry the dot+label backend badge (Claude/Codex), mono cost, AA-contrast dark theme | ✅ |

**DoD:** launch → pump → registry → live views, both backends, zero-leak teardown — proven end-to-end
in code. Remaining for a true daily-driver: the live-content render-and-look (Space-blocked right now)
and a real-binary manual run (the scripted stand-ins are deterministic CI proxies for `claude`/`codex`).
