# Feature: Frame coalescing

> User story: As Regatta, I want bursts of token-delta events coalesced before they hit the Channel, so
> the Focus pane stays at 60fps under a firehose — without ever hiding a priority approval.
> Intake: M8 — the real-time stream's backpressure.

## Requirement R-FRAME-01 — coalesce a burst, preserving priority

WHEN coalescing a burst of events THEN runs of adjacent AssistantText SHALL merge into one frame, and any other event SHALL break the run and stay a discrete, in-order frame.

@check kind=unit ref=regatta_core::stream::tests::coalesces_adjacent_text_only

WHEN a burst contains an ApprovalRequested THEN it SHALL stay a discrete, in-order frame — never merged or dropped.

@check kind=unit ref=regatta_core::stream::tests::never_coalesces_an_approval
