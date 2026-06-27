# Regatta

A fast, native desktop cockpit for running many parallel AI coding-agent CLI sessions
(**Claude Code** and **Codex**) across multiple projects — built to replace cmux without
its memory/CPU bloat. *(A regatta is a fleet of boats racing in parallel — your agents.)*

- **Attention routing first** — instantly see which of your ten sessions needs you.
- **Lean & native** — Tauri + Rust core; targets <600 MB idle, zero leaked processes.
- **Spec-driven & antidrift** — every requirement is an EARS spec bound to a passing test,
  proven through a Verified Traceability Graph (dev-spec-kit / `rivet`).

## Layout
- `regatta-core/` — pure, deterministic logic (no I/O), crate `regatta_core`, unit-tested to 100% coverage.
- `src-tauri/` — the Tauri app: thin glue (PTY, IPC, OS) wiring the core to the UI.
- `src/` — SolidJS frontend.
- `.dev-spec-kit/` — specs, laws, and the proof ledger.

## Develop
```sh
rivet() { node ~/Github/llm-dev-kit/dist/cli/index.js "$@"; }   # the antidrift CLI
cargo nextest run                       # tests
cargo llvm-cov --package regatta_core   # coverage (100% on the core)
cargo clippy --all-targets -- -D warnings
cargo tauri dev                         # run the app
```

See the per-slice loop and hard rules in `.dev-spec-kit/laws.md`.
