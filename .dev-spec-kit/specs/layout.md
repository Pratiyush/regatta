# Feature: Split-pane layout model

> User story: As Regatta, I want to arrange live sessions into a 1/2/4-pane grid, so I can watch and
> compare several at once without becoming a window manager.
> Intake: M4 — the spatial Grid.

## Requirement R-LAYOUT-01 — grid layout and pane assignment

WHEN choosing a layout from a count THEN the system SHALL map 1 → Focus (1 pane), 2 → Split (2 panes), 3+ → Quad (4 panes).

@check kind=unit ref=regatta_core::layout::tests::layout_from_count

WHEN filling panes THEN the system SHALL assign the first N session ids to the panes, padding empty slots with None.

@check kind=unit ref=regatta_core::layout::tests::fills_panes_with_sessions
