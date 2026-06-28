# Feature: Approval response

> User story: As Regatta, I want the dock's Approve/Deny turned into the exact JSON the agent's
> permission tool expects, so the decision actually drives the agent.
> Intake: M8 — the approval round-trip's response half.

## Requirement R-APPROVAL-RESP-01 — build the approve-tool response

WHEN the dock allows THEN the system SHALL return `{behavior:"allow",updatedInput:<input>}` (null input if malformed).

@check kind=unit ref=regatta_core::approval::tests::allow_echoes_input_as_updated

WHEN the dock denies THEN the system SHALL return `{behavior:"deny",message:<reason>}`.

@check kind=unit ref=regatta_core::approval::tests::deny_carries_the_reason
