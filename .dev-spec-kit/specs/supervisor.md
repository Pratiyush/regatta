# Feature: Session supervisor

> User story: As Regatta, I want every agent process to run in its own process group and be reaped
> completely on shutdown, so I never leak helpers the way cmux does (helpers stuck at ~100% CPU for days).
> Intake: M1 — the reliability promise; runs the launch plan and guarantees teardown.

## Requirement R-SUPERVISOR-01 — terminate the whole process group

WHEN a session is shut down THEN the supervisor SHALL terminate the entire process group — the agent and any child processes it spawned — leaving no surviving process.

@check kind=integration ref=regatta_supervisor::tests::shutdown_kills_the_whole_group

IF shutdown is called on a session whose process has already exited THEN it SHALL complete safely and idempotently, never panicking.

@check kind=integration ref=regatta_supervisor::tests::shutdown_is_safe_when_already_exited

## Security

A leaked or runaway agent process is a resource and trust hazard, so teardown SHALL leave no surviving process in the group — no agent ever outlives its session.

@check kind=integration ref=regatta_supervisor::tests::shutdown_kills_the_whole_group
