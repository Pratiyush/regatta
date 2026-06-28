# Feature: Config store

> User story: As Regatta, I want config layers persisted by scope (global / project / session), so
> "one system, one team" settings survive restarts and feed the resolver.
> Intake: M5 — durable config layers.

## Requirement R-CONFIGSTORE-01 — persist config layers by scope

WHEN config is set in a scope THEN `get_layer(scope)` SHALL return that scope's key→value settings.

@check kind=integration ref=regatta_store::tests::persists_config_layers_by_scope

IF the same scope + key is set again THEN the value SHALL be updated in place.

@check kind=integration ref=regatta_store::tests::set_config_updates_in_place
