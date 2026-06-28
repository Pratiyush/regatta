# Feature: Auto-pause

> User story: As Regatta, I want a runaway session to stop itself when it crosses its budget ceiling,
> so an unattended agent can't burn money — the safety net cmux never had.
> Intake: M3 — ties the budget model to the supervisor's teardown.

## Requirement R-AUTOPAUSE-01 — auto-pause a runaway session

WHEN a session's spend crosses its budget ceiling and the budget action is Block THEN the system SHALL tear the session down (auto-pause).

@check kind=integration ref=regatta_supervisor::tests::autopauses_when_budget_exceeded_and_block

IF the budget action is not Block THEN the system SHALL NOT pause even when the ceiling is exceeded.

@check kind=integration ref=regatta_supervisor::tests::does_not_autopause_when_action_is_not_block
