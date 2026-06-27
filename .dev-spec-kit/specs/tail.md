# Feature: Transcript tailer

> User story: As the supervisor, I want to read a session's transcript file from a saved byte offset,
> so live view and reattach-after-restart share one mechanism and never re-read the whole file.
> Intake: M2 — the spine of resume & crash recovery.

## Requirement R-TAIL-01 — tail a transcript from an offset

WHEN tailing a transcript file from a byte offset THEN the system SHALL read only the new complete lines, parse each into normalized events, and return the events plus the next offset — holding a trailing partial (newline-less) line until it completes.

@check kind=integration ref=regatta_supervisor::tests::tail_transcript_reads_incrementally_from_offset

IF the offset is beyond the file's current length (the file was truncated or rotated) THEN the system SHALL restart from the beginning.

@check kind=integration ref=regatta_supervisor::tests::tail_transcript_restarts_after_truncation
