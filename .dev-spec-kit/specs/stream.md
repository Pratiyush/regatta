# Feature: Claude stream-json parser

> User story: As the cockpit, I want to turn Claude Code's stream-json output into normalized
> events, so every view (Focus, Inspector, cost) reads one shape regardless of backend.
> Intake: M1 — the data spine that feeds the live session view and the cost ledger.

## Requirement R-PARSE-01 — normalize Claude stream-json lines

WHEN a Claude stream-json line of a known type arrives THEN the system SHALL emit the matching normalized event: a `system`/`init` line becomes SessionStarted (with the model), an `assistant` line becomes AssistantText (its text blocks concatenated), and a `result` line becomes Usage (cost and token counts).

@check kind=unit ref=regatta_core::stream::tests::parses_known_lines

IF a line is not valid JSON, has no recognized type, or is an unhandled type THEN the system SHALL return None and never panic.

@check kind=unit ref=regatta_core::stream::tests::ignores_malformed_and_unknown

## Security

The parser consumes untrusted process output, so it SHALL never panic and SHALL drop malformed or unrecognized lines (returning `None`) rather than emit a bogus event.

@check kind=unit ref=regatta_core::stream::tests::ignores_malformed_and_unknown
