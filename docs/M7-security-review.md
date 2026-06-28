# M7 — Go Live: payload-executing adversarial review

M7 adds no new untrusted-input *parser* (events flow from the M6-fuzzed parsers). The new surface is
the live reducer/registry (long-lived accumulation) and the `launch_session` command. Both were
attacked — see `regatta-core/tests/adversarial_runtime.rs`.

## Attack surface (new in M7)
1. **Live reducer/registry** — folds an unbounded event stream into long-lived counters (overflow risk).
2. **launch_session command** — spawns a process from a UI-supplied backend / model / id / cwd.

## Attacks executed & results
| Angle | Payload | Result |
|-------|---------|--------|
| Integer overflow | 5× `Usage{input,output: u64::MAX}` folded into one session | counters saturate at `u64::MAX` (`saturating_add`) — no wrap/panic |
| Memory / scale | 5,000 sessions × 100 KB assistant text; full `snapshot()` | no panic; snapshot total; summaries total over all |
| Command injection | (proven M6) `launch_session` → `Backend::plan_launch` builds argv; supervisor execs directly, no shell | inert — a hostile id/model is exactly one argv element |
| Process leak | a session ends / the handle drops on the pump task | process-group teardown reaps it (`claude_session_runs_live_through_the_pipeline` asserts the pid is dead) |

## Findings
**None.** The reducer uses saturating arithmetic on every counter; the registry is an unbounded
`BTreeMap` with no `unsafe`; `launch_session` validates the backend label and execs a fixed program with
discrete argv. A live session that ends is reaped by the same zero-leak teardown M1 established.

## Note
The live UI currently also serves a seeded demo set (`seed_live_demo`) alongside real launched sessions;
seeded data is folded through the same reducer, so it cannot reach a state real events could not.
