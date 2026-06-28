# M5 — One System, One Team: Adversarial Security Review

Recorded at the M5 close (2026-06-28). M5 handles config that may contain **secrets**, and masking IS a
security control. This review *executes* hostile/huge payloads and verifies the masking holds.

## New attack surface
- `regatta_core::config` — resolve, is_secret_key, mask, masked, materialize_env, effective_masked.
- `regatta_core::backend::with_env`.
- `regatta_store` config table (set_config / get_layer).
- Tauri `settings_view`.

## Findings
**None** — the ops are string/map manipulations with no overflow or unbounded recursion.

## Verified safe (executed payloads)
- **No panic on hostile/huge config.** `config_ops_never_panic_and_always_mask` runs `resolve` over
  50× 1 MB layers plus `materialize_env` / `effective_masked`; all return.
- **`mask` is multibyte-safe and length-preserving.** It counts and slices by `chars()` (not bytes), so
  unicode values mask without panicking or corrupting; `mask(v).chars().count() == v.chars().count()`
  for empty / "ab" / emoji+CJK / 1 MB inputs.
- **Secrets are always hidden.** `is_secret_key` matches KEY/TOKEN/SECRET/PASSWORD (conservative — even
  `PUBLIC_KEY` is masked); `effective_masked` never returns a secret value verbatim. `masked()` is the
  basis for "secrets masked at rest + in logs," applied wherever config is shown (`settings_view`).
- **The config store is parameterized.** `set_config` / `get_layer` bind scope/key/value via `?`
  parameters — no SQL injection.

## Standing properties
- The pure core stays I/O-free and deterministic; glue is thin and integration-tested. `cargo deny`
  green. **M5 added no third-party dependencies.**
