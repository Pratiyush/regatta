# Feature: Layered config resolver

> User story: As Regatta, I want one config layered global → project → session, so a setting changed
> once is inherited everywhere unless a narrower layer overrides it ("one system, one team").
> Intake: M5 — the shared-config spine.

## Requirement R-CONFIG-01 — resolve layered config

WHEN resolving config layers in order (global → project → session) THEN later layers SHALL override earlier keys.

@check kind=unit ref=regatta_core::config::tests::later_layers_override_earlier

WHEN there are no layers THEN the effective config SHALL be empty.

@check kind=unit ref=regatta_core::config::tests::empty_layers_resolve_empty
