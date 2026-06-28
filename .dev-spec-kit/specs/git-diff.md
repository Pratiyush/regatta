# Feature: git diff numstat parser

> User story: As Regatta, I want each changed file's +/- line counts, so the diff drawer and Review
> Inbox can show the size of a change at a glance.
> Intake: M4 — the review surface.

## Requirement R-DIFF-01 — parse git diff --numstat

WHEN parsing `git diff --numstat` THEN the system SHALL emit per-file added/removed counts; binary files (shown as `-`) yield None counts.

@check kind=unit ref=regatta_core::git::tests::parses_git_numstat

IF a line lacks the three tab-separated fields or has no path THEN the system SHALL skip it.

@check kind=unit ref=regatta_core::git::tests::numstat_skips_malformed_lines
