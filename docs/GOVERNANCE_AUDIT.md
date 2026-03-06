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
3. Configure repository variables for automation defaults when not using `solo` profile:
   - `GOVERNANCE_PROFILE`
   - `REQUIRED_APPROVING_REVIEW_COUNT` (optional)
   - `REQUIRE_CODE_OWNER_REVIEWS` (optional)
   - `REQUIRE_LAST_PUSH_APPROVAL` (optional)

## Manual Run

Use `workflow_dispatch` and optionally set:

- `governance_profile` (`solo`, `small-team`, `enterprise`; default `solo`)
- `required_approving_review_count` (optional override; default empty)

Example expected review policy:

- `solo`: `0` reviews, no code-owner requirement, no last-push approval requirement.
- `small-team`: `1` review, last-push approval required.
- `enterprise`: `2` reviews, code-owner + last-push approval required.
- Optional override can force a custom review count for audits.

## Failure Response

If the workflow fails:

1. Run `./scripts/verify-repo-policy.sh 900Labs/900Invoice main` locally.
2. Compare actual settings vs `docs/BRANCH_PROTECTION.md`.
3. Re-apply target policy with `./scripts/apply-repo-policy.sh 900Labs/900Invoice main`.
4. Re-run the workflow manually to confirm resolution.

## Incident Routing

The workflow automatically routes failures to GitHub Issues:

1. It creates or updates an open issue titled:
   - `Governance audit failure: policy drift detected`
2. It uses labels:
   - `governance-audit`
   - `incident`
   - `automation`
3. If an incident issue already exists, it appends a comment with:
   - workflow run URL
   - check outcomes
   - expected governance profile + review override input
   - remediation checklist

This keeps governance failures visible even when maintainers are not actively monitoring Actions runs.
