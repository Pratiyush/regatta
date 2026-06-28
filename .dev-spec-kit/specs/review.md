# Feature: Review summary

> User story: As Regatta, I want a one-glance "+X / −Y" summary of a session's changes, so the Review
> Inbox shows the size of each session's pending work.
> Intake: M4 — powers the Review Inbox badges + the diff drawer header.

## Requirement R-REVIEW-01 — summarize a diff for the Review Inbox

WHEN summarizing diff stats THEN the system SHALL sum the added and removed lines across files, counting binary files (no counts) as zero.

@check kind=unit ref=regatta_core::git::tests::sums_diff_stats

WHEN there are no changes THEN the summary SHALL be (0, 0).

@check kind=unit ref=regatta_core::git::tests::empty_diff_is_zero
