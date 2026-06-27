# Feature: Flow store

> User story: As Regatta, I want session metadata persisted in SQLite, so sessions survive a restart
> and the Resume board can list them.
> Intake: M1 — the durable store seed (foundation for M2 resume).

## Requirement R-STORE-01 — persist and list sessions

WHEN a session is upserted into the store THEN the system SHALL persist it and return it from a list of all sessions ordered by id.

@check kind=integration ref=regatta_store::tests::persists_and_lists_sessions

IF a session with the same id is upserted again THEN the system SHALL update it in place rather than create a duplicate.

@check kind=integration ref=regatta_store::tests::upsert_updates_existing_session
