# Governance Audit Workflow

This document defines the automated governance drift audit for repository settings and policy documentation parity.

## Workflow

- File: `.github/workflows/governance-audit.yml`
- Name: `Governance Audit`
- Triggers:
  - Weekly schedule: Monday at `03:17 UTC`
  - Manual: `workflow_dispatch`

## What It Verifies

1. Repository and branch-protection policy through:
   - `STRICT=1 ./scripts/verify-repo-policy.sh 900Labs/900Invoice main`
2. API command documentation parity through:
   - `./scripts/verify-api-doc-commands.sh`

## Token Behavior

The workflow uses:

1. `secrets.GH_ADMIN_TOKEN` when present.
2. Falls back to `${{ github.token }}` when the secret is not configured.

Recommended:

1. Configure `GH_ADMIN_TOKEN` as a fine-grained PAT with repository administration read access to avoid permission drift across GitHub settings changes.
2. Keep branch-protection configuration codified through `scripts/apply-repo-policy.sh`.

## Manual Run

Use `workflow_dispatch` and optionally set:

- `required_approving_review_count` (default `0`)

Example expected review policy:

- `0` for autonomous maintainer mode.
- `1` or higher for team-reviewed mode.

## Failure Response

If the workflow fails:

1. Run `./scripts/verify-repo-policy.sh 900Labs/900Invoice main` locally.
2. Compare actual settings vs `docs/BRANCH_PROTECTION.md`.
3. Re-apply target policy with `./scripts/apply-repo-policy.sh 900Labs/900Invoice main`.
4. Re-run the workflow manually to confirm resolution.
