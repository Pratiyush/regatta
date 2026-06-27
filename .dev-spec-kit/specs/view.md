# Feature: Event display lines

> User story: As someone watching a session, I want each agent event shown as a clear, human-readable
> line, so the Focus stream is easy to read at a glance.
> Intake: M1 — feeds the cockpit's live session view.

## Requirement R-VIEW-01 — render an event as a display line

WHEN turning a normalized event into a display line THEN the system SHALL map SessionStarted to a `system` line naming the model, AssistantText to a `claude` line carrying its text, and Usage to a `usage` line showing cost and token counts.

@check kind=unit ref=regatta_core::view::tests::maps_each_event_kind

IF an assistant event has empty text THEN the line SHALL preserve the empty text rather than drop the turn.

@check kind=unit ref=regatta_core::view::tests::preserves_empty_assistant_text
