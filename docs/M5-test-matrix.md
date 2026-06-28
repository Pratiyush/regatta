# M5 — One System, One Team: 7-type Test Matrix

Recorded at the M5 close (2026-06-28). M5 made config a shared system: layered resolve, secret masking,
env materialization, a config store, the Settings/Extensions UI, and apply-to-sessions.

| # | Type | M5 coverage | Status |
|---|------|-------------|--------|
| 1 | **Unit 100%** | `regatta_core::config` (resolve / is_secret_key / mask / masked / materialize_env / effective_masked) + `backend::with_env` at 100%. | ✅ |
| 2 | **Edge/boundary** | later-layer override; mask ≤4 fully vs >4 tail; `env.` prefix filter; missing `local_model_url`; empty layers. | ✅ |
| 3 | **Invalid/untrusted** | `config_ops_never_panic_and_always_mask` — 50× 1 MB layers; unicode/empty/huge mask (length-preserving); secret always hidden. | ✅ |
| 4 | **Scenario/integration** | config store set/get by scope + update-in-place; the chain `get_layer`×3 → resolve → materialize_env → with_env. | ✅ |
| 5 | **E2e-smoke** | app builds + launches; `settings_view` returns the effective masked config + MCP/skills/commands. | ✅ |
| 6 | **Performance/scale** | resolve/mask/materialize are O(entries); the config store is indexed by PK(scope,key). | ✅ |
| 7 | **UI/UX·a11y** | Settings + Extensions view built (effective config with masked secrets, MCP/skills switches, command chips). | ⚠️ render-and-look **capture deferred** — the window opened on a macOS Space the headless screencapture couldn't reach (3 attempts); the view compiles and reuses the render patterns of the four captured views (`m2/m3/m4-*.png`). |
