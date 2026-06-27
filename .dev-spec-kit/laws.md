# Project Laws

> The rules dev-spec-kit must always obey for this project. Three scopes are supported (Kiro-style steering):
> always-on (this file), file-match, and on-summon. A personal default can be inherited and overridden here.

## Standards
- Spec-driven & antidrift: every requirement is an EARS spec bound to a passing `@check`; ship only when the VTG is green (`rivet gate --quiet`).
- Pure core / thin glue: all logic in `regatta_core` (100% covered, deterministic — inject `Clock`/`Rng`); `src-tauri` is thin, e2e-verified glue.
- Small, frequent, reviewed releases via `release-please`; protected `main`; squash-merge.
- Performance is a feature: target <600 MB idle and **zero leaked processes** — regressions fail CI.

## Tech & conventions
- Stack: Tauri 2 + Rust core; SolidJS + TypeScript + Vite frontend; SQLite (`sqlx`) for the flow store.
- Layout: `regatta-core/` (pure lib `regatta_core`), `src-tauri/` (Tauri app), `src/` (frontend), `.dev-spec-kit/` (specs, laws, ledger).
- Backends behind one `AgentBackend` trait (Claude Code + Codex); every view consumes the normalized event model.
- Naming: `R-<AREA>-NN` requirements; check refs `regatta_core::<module>::tests::<fn>`; session branches `regatta/<slug>-<uuid>`; Conventional Commits.

## Hard rules (never violate)
- Commits are authored by the human, never co-authored by the AI.
- A task is not "done" until its bound checks pass (evidence-bound completion).
- Reuse existing code before writing new; follow the surrounding patterns.
- The Tauri command boundary is the trust boundary — validate every input; least-privilege allowlist + strict CSP.
