# Feature: Session env materializer

> User story: As Regatta, I want the effective config turned into the environment a session launches
> with — including the local-model path — so config set once actually reaches the agent.
> Intake: M5 — config → session.

## Requirement R-MATERIALIZE-01 — materialize config into session env

WHEN materializing the effective config into session env THEN keys prefixed `env.` SHALL become environment variables, and `local_model_url` SHALL set `ANTHROPIC_BASE_URL` (the local-model path).

@check kind=unit ref=regatta_core::config::tests::materializes_env_with_local_model_path

WHEN there is no `local_model_url` THEN no `ANTHROPIC_BASE_URL` SHALL be set.

@check kind=unit ref=regatta_core::config::tests::no_local_model_means_no_base_url
