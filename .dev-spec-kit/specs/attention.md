# Feature: Attention priority

> User story: As someone running ten agents at once, I want the cockpit to rank sessions by how
> urgently they need me, so the Attention Dock surfaces the most urgent first and working sessions stay out of the way.
> Intake: M1 — the headline "triage over tabs" feature.

## Requirement R-ATTN-01 — rank sessions that need the human

WHEN scoring a session that is blocked on the human THEN the system SHALL return a priority of 5 for WaitingApproval, 4 for NeedsInput, 3 for Error, and 2 for an unseen Done — so the Attention Dock sorts the most urgent first.

@check kind=unit ref=regatta_core::attention::tests::ranks_blocked_sessions

IF a session is actively working or transitional (Starting, Running, Compacting, RateLimited, Paused) THEN the system SHALL return a priority of 0 so it never surfaces in the Attention Dock.

@check kind=unit ref=regatta_core::attention::tests::working_sessions_score_zero

## Requirement R-ATTN-02 — order the Attention Dock

WHEN ordering sessions for the Attention Dock THEN the system SHALL return only the unseen sessions that need the human, most-urgent first (priority descending), preserving input order on ties.

@check kind=unit ref=regatta_core::attention::tests::dock_orders_by_urgency

IF every session is working/transitional or already seen THEN the system SHALL return an empty list so the dock shows nothing.

@check kind=unit ref=regatta_core::attention::tests::dock_excludes_working_and_seen

