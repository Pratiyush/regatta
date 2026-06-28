# Feature: git status parser

> User story: As Regatta, I want each session's working-tree changes as structured data, so the file
> tree and Review Inbox can show what changed (M/A/D) without me leaving the app.
> Intake: M4 — the review surface.

## Requirement R-GITSTATUS-01 — parse git status --porcelain

WHEN parsing `git status --porcelain` THEN the system SHALL emit one FileChange per file with its status code (M / A / D / R / ?); a rename's path is the new name.

@check kind=unit ref=regatta_core::git::tests::parses_git_status_porcelain

IF a line is blank, too short, or has no path THEN the system SHALL skip it.

@check kind=unit ref=regatta_core::git::tests::skips_blank_and_pathless_lines
