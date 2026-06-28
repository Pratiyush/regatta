# M4 — Review & Multi-pane Orchestration: 7-type Test Matrix

Recorded at the M4 close (2026-06-28). M4 made review fast and sessions spatial: git status/diff
parsers, a git reader, a split-pane layout model, and the Review view.

| # | Type | M4 coverage | Status |
|---|------|-------------|--------|
| 1 | **Unit 100%** | `regatta_core` (git: `parse_status`/`parse_numstat`/`summarize_diff`; layout: `GridLayout`/`panes`) at 100% (981 regions). | ✅ |
| 2 | **Edge/boundary** | rename `old -> new` path; binary (`-`) numstat; pathless/short lines skipped; `from_count` boundaries (1/2/3+); empty diff → (0,0). | ✅ |
| 3 | **Invalid/untrusted** | `git_parsers_and_summary_never_panic` — traversal / null-byte / control-char / 1 MB-path / garbage input; `summarize_diff` saturates u64. | ✅ |
| 4 | **Scenario/integration** | `regatta_git` reads a real temp repo (M/A status + numstat); a non-repo → empty. | ✅ |
| 5 | **E2e-smoke** | app builds + launches; the Review view renders the split grid + inbox + diff drawer. | ✅ |
| 6 | **Performance/scale** | parsers are O(lines); git runs once per refresh; layout is O(1). | ✅ |
| 7 | **UI/UX·a11y** | render-and-look of the Review view: quad split-grid, Review Inbox (+/- per session), diff drawer (M/A/D badges, per-file counts, binary flag). | ✅ |

Artifact: `scratchpad/m4-review.png`.
