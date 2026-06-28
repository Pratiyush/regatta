# Feature: Approval round-trip

> User story: As Regatta, I want an agent's approve request, the dock's decision, and the agent's reply
> to compose into one round-trip, so Approve/Deny actually drives a real session.
> Intake: M8 — the inline-approval round-trip.

## Requirement R-APPROVAL-RT-01 — the round-trip drives the agent

WHEN an agent's approve request is parsed THEN the dock SHALL surface ApprovalRequested; WHEN the dock allows THEN the agent SHALL receive `{behavior:"allow",updatedInput:<input>}` (proceed).

@check kind=integration ref=regatta_core::approval::tests::allow_round_trip_drives_the_agent

WHEN the dock denies THEN the agent SHALL receive `{behavior:"deny",message:<reason>}` (stop).

@check kind=integration ref=regatta_core::approval::tests::deny_round_trip_stops_the_agent

> Note: the in-process MCP permission server that carries this over stdio to a real `claude` session is
> the remaining runtime wiring; this proves the request/decision/response logic end-to-end.
