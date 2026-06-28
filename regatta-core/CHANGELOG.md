# Changelog

## [0.5.0](https://github.com/Pratiyush/regatta/compare/v0.4.0...v0.5.0) (2026-06-28)


### Features

* **approval:** build the MCP approve-tool response ([#113](https://github.com/Pratiyush/regatta/issues/113)) ([ffe159d](https://github.com/Pratiyush/regatta/commit/ffe159d064ba7147e7264a992cdfc93b6a92a699))
* **stream:** ApprovalRequested event + parse the approve request ([#110](https://github.com/Pratiyush/regatta/issues/110)) ([61f78d0](https://github.com/Pratiyush/regatta/commit/61f78d0d5b1a23b3335d2988519b9df952f56ddf))
* **stream:** coalesce token-delta bursts, preserving priority frames ([#115](https://github.com/Pratiyush/regatta/issues/115)) ([1c09c14](https://github.com/Pratiyush/regatta/commit/1c09c147b70f257187617a00b71067c126e1c8b8))

## [0.4.0](https://github.com/Pratiyush/regatta/compare/v0.3.0...v0.4.0) (2026-06-28)


### Features

* **live:** launch_session + live_sessions wire the registry into Tauri ([#104](https://github.com/Pratiyush/regatta/issues/104)) ([7054d44](https://github.com/Pratiyush/regatta/commit/7054d44cedb36fd144b49c6f7bf657318e4f0d59))
* **runtime:** fold the event stream into live session state ([#97](https://github.com/Pratiyush/regatta/issues/97)) ([719b3d6](https://github.com/Pratiyush/regatta/commit/719b3d6e2da95350f5b1817abe7a8fbf73fd4fd0))
* **runtime:** session registry — live sessions keyed by id ([#100](https://github.com/Pratiyush/regatta/issues/100)) ([c712609](https://github.com/Pratiyush/regatta/commit/c712609f3bb2ced04123b15053fedb7286cb1c63))

## [0.3.0](https://github.com/Pratiyush/regatta/compare/v0.2.0...v0.3.0) (2026-06-28)


### Features

* **backend:** Backend dispatch (Claude/Codex) — the multi-backend abstraction ([#85](https://github.com/Pratiyush/regatta/issues/85)) ([c7a535b](https://github.com/Pratiyush/regatta/commit/c7a535b57fe7ffe5ee3b168ffc000fc127556da3))
* **backend:** plan a Codex launch (codex exec --json) ([#83](https://github.com/Pratiyush/regatta/issues/83)) ([0383c3b](https://github.com/Pratiyush/regatta/commit/0383c3b5b87985bb583de7fea2ad16a881d7441d))
* **backend:** with_env merges materialized config into a launch plan ([#75](https://github.com/Pratiyush/regatta/issues/75)) ([60eb7ee](https://github.com/Pratiyush/regatta/commit/60eb7eed5c10f3e0b83cfbc4943b7a189f0f29f9))
* **badge:** Claude/Codex backend badge on every session ([#91](https://github.com/Pratiyush/regatta/issues/91)) ([cb964df](https://github.com/Pratiyush/regatta/commit/cb964df96872d4aa2b14f4387157be5cde2d24b7))
* **budget:** classify spend and decide auto-pause ([#45](https://github.com/Pratiyush/regatta/issues/45)) ([782c97e](https://github.com/Pratiyush/regatta/commit/782c97e4eb40b5c9dca1ef110eec7c3475c60347))
* **config:** layered config resolver (global-&gt;project-&gt;session) ([#67](https://github.com/Pratiyush/regatta/issues/67)) ([cf082d6](https://github.com/Pratiyush/regatta/commit/cf082d69dbf1c5b68cef063d74017322c722ebd9))
* **config:** mask secret config values ([#69](https://github.com/Pratiyush/regatta/issues/69)) ([3ec447c](https://github.com/Pratiyush/regatta/commit/3ec447c62e5bced30974f77a9d1931776d35ddf6))
* **config:** materialize effective config into session env ([#71](https://github.com/Pratiyush/regatta/issues/71)) ([48d3d4e](https://github.com/Pratiyush/regatta/commit/48d3d4e701d02f027c4e7e934a4655519baf158c))
* **cost:** burn rate and time-to-ceiling prediction ([#43](https://github.com/Pratiyush/regatta/issues/43)) ([4b5d092](https://github.com/Pratiyush/regatta/commit/4b5d092ec670a82cc94fc587c5947151a95c6bec))
* **cost:** price token usage and pick effective session cost ([#40](https://github.com/Pratiyush/regatta/issues/40)) ([9b0cf29](https://github.com/Pratiyush/regatta/commit/9b0cf29d3d3ec67d307b40cf7fb3c940cf4ba3c0))
* **git:** parse git diff --numstat into per-file line counts ([#57](https://github.com/Pratiyush/regatta/issues/57)) ([244409f](https://github.com/Pratiyush/regatta/commit/244409f3b660757894cea0c83ebfbf4bd4b38011))
* **git:** parse git status --porcelain into file changes ([#55](https://github.com/Pratiyush/regatta/issues/55)) ([ac62888](https://github.com/Pratiyush/regatta/commit/ac62888721ba597cb7bba658f949410c171342d9))
* **layout:** grid layout model + pane assignment ([#59](https://github.com/Pratiyush/regatta/issues/59)) ([7ec706c](https://github.com/Pratiyush/regatta/commit/7ec706c830d1053364e1019deb2bfe0ac3865706))
* **review:** Review Inbox + diff drawer + split grid (M4.5+M4.6) ([#63](https://github.com/Pratiyush/regatta/issues/63)) ([2f9e1a8](https://github.com/Pratiyush/regatta/commit/2f9e1a86b4aa5bb50be10a780ee83d5fe6636cbe))
* **settings:** Settings + Extensions view (one system, one team) ([#77](https://github.com/Pratiyush/regatta/issues/77)) ([9d11152](https://github.com/Pratiyush/regatta/commit/9d1115200b83f53bc4895daf0d38385ec8b4e7d0))
* **stream:** parse Codex events into the normalized shape ([#81](https://github.com/Pratiyush/regatta/issues/81)) ([4f65646](https://github.com/Pratiyush/regatta/commit/4f65646de350fb93cc260f6085e3970ff0a76b72))
* **transcript:** parse Codex session meta for the Resume board ([#87](https://github.com/Pratiyush/regatta/issues/87)) ([246fe2c](https://github.com/Pratiyush/regatta/commit/246fe2c0ea591920bcd7632dfaf5d0e48cf69ad3))
* **usage:** live Usage view — spend, burn, budget bar, by-project/model ([#49](https://github.com/Pratiyush/regatta/issues/49)) ([32f3a69](https://github.com/Pratiyush/regatta/commit/32f3a69c49a1f2c72fb77d079f50819f334f9543))


### Bug Fixes

* harden summarize_diff against u64 overflow (M4 adversarial review) ([#65](https://github.com/Pratiyush/regatta/issues/65)) ([a15288c](https://github.com/Pratiyush/regatta/commit/a15288c64773b6b3f65a1e832e5973d99a92b5ae))

## [0.2.0](https://github.com/Pratiyush/regatta/compare/v0.1.0...v0.2.0) (2026-06-27)


### Features

* **attention:** dock_order — rank the Attention Dock ([#6](https://github.com/Pratiyush/regatta/issues/6)) ([d8cb130](https://github.com/Pratiyush/regatta/commit/d8cb1302ac28a0ee42052270abc44b40ad96c070))
* **attention:** priority scoring for the Attention Dock ([#3](https://github.com/Pratiyush/regatta/issues/3)) ([eed81c7](https://github.com/Pratiyush/regatta/commit/eed81c76cf0e0918f57a4df5d2842255555145fe))
* **backend:** plan_resume + resume_command for the Resume board ([#28](https://github.com/Pratiyush/regatta/issues/28)) ([10bd468](https://github.com/Pratiyush/regatta/commit/10bd46806e194a44e955b34234b8894ddee96e83))
* **backend:** pure Claude launch plan (argv/env) for the supervisor ([#10](https://github.com/Pratiyush/regatta/issues/10)) ([c78266e](https://github.com/Pratiyush/regatta/commit/c78266e9b17ac4edfc8d962a3b3252ed0b6db303))
* **board:** Resume board UI with recency grouping and one-click resume ([#36](https://github.com/Pratiyush/regatta/issues/36)) ([1672ab0](https://github.com/Pratiyush/regatta/commit/1672ab00c6f64f35d6209228d48293db7cbd23cc))
* **cockpit:** run a live session end-to-end + human-friendly UI ([#16](https://github.com/Pratiyush/regatta/issues/16)) ([f0f5be6](https://github.com/Pratiyush/regatta/commit/f0f5be61ac925d11d8b94a410fa253b87203ec3d))
* **stream:** Claude stream-json parser -&gt; normalized events ([#8](https://github.com/Pratiyush/regatta/issues/8)) ([5d83261](https://github.com/Pratiyush/regatta/commit/5d83261d9e4196e16bffac8674376e0f85bcea31))
* **transcript:** parse Claude transcript session metadata ([#24](https://github.com/Pratiyush/regatta/issues/24)) ([64ff8dd](https://github.com/Pratiyush/regatta/commit/64ff8dd7aa0cf019b7f242779b9b5593df42d0d9))
* **worktree:** plan a unique per-session git worktree ([#18](https://github.com/Pratiyush/regatta/issues/18)) ([ece4d8c](https://github.com/Pratiyush/regatta/commit/ece4d8cca0b3ddd5a77c8577bdfacd1922d78874))


### Bug Fixes

* harden recency overflow and indexer symlink recursion (M2 adversarial review) ([#38](https://github.com/Pratiyush/regatta/issues/38)) ([839082f](https://github.com/Pratiyush/regatta/commit/839082f2b9aba62c144bb0666b981e33de919d70))

## 0.1.0 (2026-06-27)


### Features

* bootstrap Regatta — Tauri+Rust scaffold, antidrift harness, slugify core ([675bb63](https://github.com/Pratiyush/regatta/commit/675bb633e393de8c8038aa8a4ce4cd5496363ae1))


### Bug Fixes

* **ci:** point release-please at the regatta-core crate with a literal version ([83915f1](https://github.com/Pratiyush/regatta/commit/83915f1ce2a05e86d765f8b7f202971b9da34529))
