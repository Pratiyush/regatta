# Feature: Flow store v2 — transcript index

> User story: As the Resume board, I want a durable, searchable index of every session, so I can find
> and resume any of them fast (35+ sessions, <50 ms search).
> Intake: M2 — the persistence behind the board.

## Requirement R-STORE-02 — persist and search transcripts

WHEN a transcript is upserted into the index THEN the system SHALL persist it and return it from a board search by title or project, ordered most-recent first.

@check kind=integration ref=regatta_store::tests::transcript_index_persists_and_searches

IF the same session id is upserted again THEN the system SHALL update it in place rather than create a duplicate.

@check kind=integration ref=regatta_store::tests::upsert_transcript_updates_in_place
