# Feature: Approval request parsing

> User story: As Regatta, I want an agent's tool-use approval request normalized into one event the dock
> can surface, so inline Approve/Deny works for any backend.
> Intake: M8 — inline approvals.

## Requirement R-APPROVAL-PARSE-01 — parse an approval request

WHEN parsing the input the `mcp__regatta__approve` permission tool receives THEN the system SHALL produce `ApprovalRequested{tool, detail}` — detail is the command for a Bash request, else the rendered input.

@check kind=unit ref=regatta_core::stream::tests::parses_an_approval_request

IF the input lacks a tool name or is malformed THEN the system SHALL return None.

@check kind=unit ref=regatta_core::stream::tests::rejects_non_approval_input
