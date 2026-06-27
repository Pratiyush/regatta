# Feature: Resume planning

> User story: As the Resume board, I want to relaunch a past session and offer a copy-paste resume
> command, so a session can be picked up exactly where it left off.
> Intake: M2 — the resume action behind the board.

## Requirement R-RESUME-01 — resume an existing session

WHEN resuming a session THEN the system SHALL produce a launch plan that attaches to it via `--resume`, and a copy-paste command `claude --resume <id>` for the board's Copy button.

@check kind=unit ref=regatta_core::backend::tests::plans_a_resume_and_command

IF the session id is empty THEN the resume command SHALL still be well-formed (`claude --resume `) and never panic.

@check kind=unit ref=regatta_core::backend::tests::resume_command_handles_empty_id
