# Feature: Resume board

> User story: As the Resume board, I want sessions grouped by recency, so I can scan my history the
> way I think about it (today, yesterday, earlier) instead of one flat list.
> Intake: M2 — the board's recency rail.

## Requirement R-BOARD-01 — group sessions by recency

WHEN grouping a session for the board THEN the system SHALL bucket it by how long ago it was last active: Today, Yesterday, Earlier this week, Last two weeks, or Older.

@check kind=unit ref=regatta_core::board::tests::buckets_by_recency

IF the last-activity time is in the future (clock skew) THEN the system SHALL treat it as Today rather than a negative bucket.

@check kind=unit ref=regatta_core::board::tests::future_or_zero_diff_is_today
