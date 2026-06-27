# M1 — 7-type test matrix (milestone close)

Recorded at M1 close. "Done" = covered now; "Partial" = some coverage + a logged gap; "Gap" = deferred (with where).

| # | Type | Status | Evidence / gap |
|---|------|--------|----------------|
| 1 | **Unit 100%** | ✅ Done | `regatta_core` at **100% line coverage** (CI-enforced, `cargo-llvm-cov --fail-under-lines 100`); 23 VTG-bound unit tests across attention/stream/backend/view/slug/worktree. |
| 2 | **Edge / boundary** | ◑ Partial | Boundaries tested: empty slug→`session`, short uuid, empty assistant text, malformed/garbage JSON, no-usage result. Plus adversarial payloads (huge/Unicode/overflow). **Gap:** no `proptest` property-based fuzzing yet → logged for M2/M3. |
| 3 | **Invalid / untrusted** | ✅ Done | `regatta-core/tests/adversarial.rs` **executes** shell-injection, path-traversal, 2 MB lines, 20k-block JSON, numeric overflow, null bytes through the pure surfaces — parser never panics, slug/worktree sanitized. Tauri command inputs are typed (no raw strings reach a shell; supervisor args are a `Vec`). |
| 4 | **Scenario / integration** | ✅ Done | Supervisor integration tests (spawn child+grandchild → **zero survivors**; idempotent shutdown; stdout→events pipeline); SQLite store in-memory integration; `run_demo_session` exercises the full plan→spawn→parse→view pipeline. |
| 5 | **E2e-smoke** | ◑ Partial | Packaged app launches + headline flow (a live session renders) verified by **render-and-look** (two screenshots). **Gap:** no automated `tauri-driver`/WebDriver e2e yet → logged for M2. |
| 6 | **Performance / scale** | ○ Gap | Lean release profile + bounded design in place, but **no `criterion` bench or RSS/60fps soak gate measured yet** → logged for M2 (when the UI drives many concurrent live streams). The architecture's soak/RSS CI gate is the target. |
| 7 | **UI/UX · a11y** | ◑ Partial | Render-and-look done; status uses **dot + label** (color-blind-safe); keyboard hints shown (⌘⇧U etc.). **Gap:** no formal ARIA/contrast/reduced-motion audit or keyboard-only pass yet → logged for M2 UI hardening. |

**Summary:** 3 fully done (unit, invalid/untrusted, integration), 3 partial (edge, e2e, a11y), 1 gap (perf). Gaps are deliberate and tracked for M2; none block M1.
