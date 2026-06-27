# Changelog

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
