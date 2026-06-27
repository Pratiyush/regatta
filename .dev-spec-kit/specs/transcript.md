# Feature: Transcript metadata

> User story: As the indexer, I want to read a Claude transcript's session metadata from a single
> line, so the Resume board can list every past session without loading the whole (up to 44 MB) file.
> Intake: M2 — the parse step that powers indexing and resume.

## Requirement R-TXMETA-01 — parse Claude transcript session metadata

WHEN parsing a line of a Claude transcript THEN the system SHALL extract the session id, working directory, git branch, and a project label (the slug of the working directory's last path segment).

@check kind=unit ref=regatta_core::transcript::tests::parses_session_meta

IF the line is not valid JSON or has no session id THEN the system SHALL return None.

@check kind=unit ref=regatta_core::transcript::tests::rejects_malformed_or_idless
