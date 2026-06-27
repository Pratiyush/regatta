---
inclusion: fileMatch
pattern: \.rs$
---
# Best practices — Rust (helm-core & glue)

## Pure core vs glue (the 100% rule)
- All logic lives in `helm-core` as pure functions/data: **no I/O, no Tauri, no `SystemTime::now()`, no `rand`.** Inject a `Clock` and a seeded `Rng` instead, so tests are reproducible and proofs replayable.
- `helm-core` is unit-tested to **100% line + branch** coverage (`cargo llvm-cov`). Keep modules small enough to make that real.
- Glue (`src-tauri/`: PTY, process, IPC, FS, webview) is coverage-excluded and verified by integration + e2e tests.

## Safety & errors
- `#![forbid(unsafe_code)]` in `helm-core`; justify any `unsafe` in glue with a comment.
- No `unwrap()`/`expect()`/`panic!` on any path reachable from untrusted input (transcripts, IPC payloads, CLI output). Return `Result`; model errors as an enum (`thiserror`).
- Validate every value crossing the trust boundary (Tauri command args, parsed CLI JSON) before use.

## Tests
- Bind every requirement criterion to a `#[test]` via `@check kind=unit ref=helm_core::<module>::tests::<fn>`.
- Property-test parsers and scorers with `proptest`; table-test edges: empty · huge · malformed JSON · Unicode · truncated stream.

## Style
- `cargo fmt` and `cargo clippy --all-targets -- -D warnings` are part of "done".
- Prefer iterators and exhaustive `match`; avoid `as` casts that can truncate.
