# Feature: git reader

> User story: As Regatta, I want a session's real working-tree changes and diff stats from its repo,
> so the file tree + diff drawer show live, accurate review data.
> Intake: M4 — runs git and feeds the pure parsers.

## Requirement R-GITREAD-01 — read a repo's status and diffstat

WHEN reading a git repo THEN `status()` SHALL return its working-tree changes and `diffstat()` the per-file +/- counts (via the pure parsers).

@check kind=integration ref=regatta_git::tests::reads_status_and_diffstat_from_a_repo

IF the directory is not a git repository THEN `status()` SHALL return empty without error.

@check kind=integration ref=regatta_git::tests::non_repo_yields_empty_status
