# Feature: Transcript indexer

> User story: As Regatta, I want to index every Claude transcript under `~/.claude/projects` into the
> flow store, so the Resume board lists my whole history — reading only each file's head, never the
> whole (up to 44 MB) file.
> Intake: M2 — populates the board.

## Requirement R-INDEX-01 — index a transcript directory

WHEN scanning a directory for transcripts THEN the system SHALL index each `.jsonl` file (reading only its head) into the store's transcript index, skipping files whose first line is not valid session metadata.

@check kind=integration ref=regatta_indexer::tests::indexes_a_transcript_dir

IF the directory does not exist THEN the system SHALL return zero without error.

@check kind=integration ref=regatta_indexer::tests::missing_dir_yields_nothing
