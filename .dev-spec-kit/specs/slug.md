# Feature: Worktree slug

> User story: As Helm, I want to turn a session name into a filesystem- and git-safe slug,
> so every session gets a clean, unique worktree branch name.
> Intake: M0 dogfood — the smallest real slice that proves the antidrift loop end-to-end.

## Requirement R-SLUG-01 — slugify a session name

WHEN given a session name THEN the system SHALL return a lowercase slug containing only `[a-z0-9-]`, with each run of other characters collapsed to a single hyphen and leading and trailing hyphens removed.

@check kind=unit ref=regatta_core::slug::tests::slugifies_a_name

IF the name contains no alphanumeric characters THEN the system SHALL return the literal `session`, never an empty branch name.

@check kind=unit ref=regatta_core::slug::tests::empty_slug_falls_back
