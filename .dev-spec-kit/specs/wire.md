# Feature: Session event stream

> User story: As the cockpit, I want the supervisor to read a running agent's stdout and turn it into
> normalized events, so a live session's output flows into the Focus view and the cost ledger.
> Intake: M1 — wires the supervisor (process) to the parser (events); the real "drive a session" pipeline.

## Requirement R-WIRE-01 — collect normalized events from a session's stdout

WHEN an agent process writes stream-json lines to stdout THEN the supervisor SHALL read them to EOF and return the parsed NormalizedEvents in order.

@check kind=integration ref=regatta_supervisor::tests::collects_parsed_events_from_stdout

IF stdout contains blank or unparseable lines THEN the supervisor SHALL skip them, returning only the valid events and never crashing.

@check kind=integration ref=regatta_supervisor::tests::skips_unparseable_lines
