# Feature: Live session registry

> User story: As Regatta, I want one registry of all live sessions keyed by id, so the cockpit renders
> a single source of truth folded from the event streams.
> Intake: M7 — the live source the dock/focus read.

## Requirement R-REGISTRY-01 — track live sessions by id

WHEN applying an event to a session id THEN the registry SHALL create-or-update that id's runtime (a new id is created on first event).

@check kind=unit ref=regatta_core::runtime::tests::registry_tracks_sessions_by_id

WHEN snapshotting THEN the registry SHALL return every session's (id, runtime), id-sorted.

@check kind=unit ref=regatta_core::runtime::tests::snapshot_is_id_sorted
