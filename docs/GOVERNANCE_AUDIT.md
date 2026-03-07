# Governance Audit Workflow

This document defines the automated governance drift audit for repository settings and policy documentation parity.

## Workflow

- File: `.github/workflows/governance-audit.yml`
- Name: `Governance Audit`
- Triggers:
  - Weekly schedule: Monday at `03:17 UTC`
  - Manual: `workflow_dispatch`

Artifact inventory reporting:

- File: `.github/workflows/governance-artifact-inventory.yml`
- Name: `Governance Artifact Inventory`
- Triggers:
  - Weekly schedule: Monday at `03:47 UTC`
  - Manual: `workflow_dispatch`

## What It Verifies

1. Repository and branch-protection policy through:
   - `STRICT=1 ./scripts/verify-repo-policy.sh 900Labs/900Invoice main`
2. API command documentation parity through:
   - `./scripts/verify-api-doc-commands.sh`
3. Profile assertion artifact generation:
   - `governance-profile-assertion-<run_id>`
   - captures resolved policy inputs for triage

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
4. Optional external notification secrets:
   - `GOVERNANCE_INCIDENT_WEBHOOK_URL` (endpoint for chat/email/webhook gateway notifications)
   - `GOVERNANCE_INCIDENT_WEBHOOK_TOKEN` (optional bearer token sent as `Authorization: Bearer <token>`)
   - `GOVERNANCE_INCIDENT_WEBHOOK_HMAC_SECRET` (optional HMAC secret for signed webhook delivery)
5. Optional governance artifact retention variables:
   - shared default: `GOVERNANCE_ARTIFACT_RETENTION_DAYS` (`1`-`90`, default `30`)
   - governance-audit override: `GOVERNANCE_AUDIT_ARTIFACT_RETENTION_DAYS` (`1`-`90`)
   - inventory-report override: `GOVERNANCE_INVENTORY_ARTIFACT_RETENTION_DAYS` (`1`-`90`)
6. Optional external notification retry variables:
   - `GOVERNANCE_INCIDENT_WEBHOOK_MAX_ATTEMPTS` (`1`-`6`, default `3`)
   - `GOVERNANCE_INCIDENT_WEBHOOK_BACKOFF_SECONDS` (`1`-`30`, default `2`)

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

## Optional External Notifications

The workflow can additionally send failure notifications to an external webhook endpoint.

Configuration:

1. Set `GOVERNANCE_INCIDENT_WEBHOOK_URL` in repository secrets.
2. Optionally set `GOVERNANCE_INCIDENT_WEBHOOK_TOKEN` for bearer-authenticated receivers.
3. Optionally set `GOVERNANCE_INCIDENT_WEBHOOK_HMAC_SECRET` for signed payload delivery.
4. Optionally set retry tuning repository variables:
   - `GOVERNANCE_INCIDENT_WEBHOOK_MAX_ATTEMPTS`
   - `GOVERNANCE_INCIDENT_WEBHOOK_BACKOFF_SECONDS`

Behavior:

1. The notification step runs only when the workflow is already failing and after issue routing succeeds.
2. If no webhook URL is configured, the step exits without error.
3. The workflow retries failed webhook delivery attempts with linear backoff (`backoff_seconds * attempt`) up to the configured max attempts.
4. When `GOVERNANCE_INCIDENT_WEBHOOK_HMAC_SECRET` is configured, the workflow sends:
   - `X-Governance-Timestamp` (UTC epoch seconds)
   - `X-Governance-Signature` (`sha256=<hex>` of `<timestamp>.<payload_json>`)
5. The payload is JSON and includes:
   - event (`governance_audit_failure`)
   - repository/workflow identifiers
   - workflow run URL
   - incident issue URL and number
   - resolved governance profile
   - policy/API-doc check outcomes
   - UTC timestamp

This keeps the default governance channel transparent in GitHub Issues while allowing optional chat/email escalation through existing infrastructure.

## Profile Assertion Artifact

Each run uploads one artifact:

1. Name: `governance-profile-assertion-<run_id>`
2. File: `governance-profile-assertion.txt`
3. Fields include:
   - resolved profile
   - resolved review/code-owner/last-push flags
   - input and repository-variable override values
   - UTC timestamp
4. Retention:
   - uses `GOVERNANCE_AUDIT_ARTIFACT_RETENTION_DAYS` when set
   - otherwise falls back to `GOVERNANCE_ARTIFACT_RETENTION_DAYS`
   - workflow enforces valid range `1` to `90`

## Governance Artifact Inventory Report

The inventory workflow builds a periodic snapshot of governance artifacts for retention-policy audits.

Outputs:

1. Artifact: `governance-artifact-inventory-<run_id>`
2. Files:
   - `governance-artifact-inventory.md`
   - `governance-artifact-inventory.json`
3. Contents:
   - total/active/expired governance artifact counts
   - near-expiration counts (next 7 days)
   - most recent governance artifacts with created/expires metadata

Retention policy:

1. Uses `GOVERNANCE_INVENTORY_ARTIFACT_RETENTION_DAYS` when set.
2. Otherwise falls back to `GOVERNANCE_ARTIFACT_RETENTION_DAYS`.
3. Enforced range remains `1` to `90`.

Use this artifact during incident triage to confirm what policy contract the workflow actually evaluated.

Maintainer triage checklist is available at `docs/MAINTAINER_CHECKLIST.md`.
