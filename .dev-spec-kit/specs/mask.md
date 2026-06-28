# Feature: Secret masking

> User story: As Regatta, I want secret config values (API keys, tokens) masked everywhere they're
> shown or logged, so "one system, one team" config never leaks a credential.
> Intake: M5 — secrets masked at rest + in logs.

## Requirement R-MASK-01 — mask secret config values

WHEN a key names a secret (contains KEY / TOKEN / SECRET / PASSWORD) THEN `is_secret_key` SHALL be true, and `mask` SHALL hide all but the last 4 characters (values ≤ 4 chars fully masked).

@check kind=unit ref=regatta_core::config::tests::flags_secret_keys

WHEN masking a config THEN secret values SHALL be masked and non-secret values left intact.

@check kind=unit ref=regatta_core::config::tests::masks_secret_values
