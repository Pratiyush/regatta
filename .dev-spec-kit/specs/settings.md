# Feature: Settings view

> User story: As Regatta, I want the Settings view to show the effective config (resolved across
> layers) with secrets masked, so I see exactly what every session inherits without leaking keys.
> Intake: M5 — the Settings + Extensions surface.

## Requirement R-SETTINGS-01 — effective config display

WHEN displaying settings THEN the system SHALL show the effective config (resolved global → project → session) with secret values masked.

@check kind=unit ref=regatta_core::config::tests::effective_masked_resolves_and_masks
