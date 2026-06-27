# Feature: Reattach on startup

> User story: As Regatta, on launch I want to restore every persisted session by replaying its
> transcript from the saved offset, so nothing is lost across a restart or crash.
> Intake: M2 — crash recovery; ties the tailer to the store.

## Requirement R-REATTACH-01 — reattach persisted sessions

WHEN reattaching THEN the system SHALL replay each persisted session's events from its saved transcript offset and advance the offset, so a later reattach with no new data returns nothing new.

@check kind=integration ref=regatta_indexer::tests::reattaches_and_advances_offset

IF a session's transcript file no longer exists THEN the system SHALL skip it without error and still reattach the others.

@check kind=integration ref=regatta_indexer::tests::skips_a_missing_transcript
