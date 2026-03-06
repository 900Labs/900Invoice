# Sprint 023: Webhook Delivery and Signing Hardening

- **Sprint ID**: `023`
- **Date**: 2026-03-07
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 019 introduced optional external governance-incident webhook notifications. Remaining hardening follow-ups were retry/backoff controls and stronger authenticity guarantees for webhook receivers.

## Goals

1. Add resilient webhook delivery with bounded retry/backoff.
2. Add optional HMAC signing support for webhook authenticity verification.
3. Document configuration contract for maintainers.

## In Scope

1. Extend governance-audit workflow webhook step with retry/backoff controls.
2. Add optional HMAC signing headers for webhook delivery.
3. Validate retry/backoff input ranges in workflow runtime.
4. Update governance and maintainer docs plus changelog.

## Out of Scope

1. Product runtime behavior changes.
2. Provider-specific webhook adapters.
3. Release workflow changes.

## Deliverables

1. Updated `.github/workflows/governance-audit.yml`:
   - added `GOVERNANCE_INCIDENT_WEBHOOK_MAX_ATTEMPTS` (`1`-`6`, default `3`)
   - added `GOVERNANCE_INCIDENT_WEBHOOK_BACKOFF_SECONDS` (`1`-`30`, default `2`)
   - added optional `GOVERNANCE_INCIDENT_WEBHOOK_HMAC_SECRET` secret
   - webhook delivery now retries with linear backoff (`backoff_seconds * attempt`)
   - optional HMAC headers when secret is configured:
     - `X-Governance-Timestamp`
     - `X-Governance-Signature` as `sha256=<hex>` of `<timestamp>.<payload_json>`
2. Updated docs:
   - `docs/GOVERNANCE_AUDIT.md`
   - `docs/MAINTAINER_CHECKLIST.md`
   - `CHANGELOG.md`

## Validation

1. `ruby -ryaml -e 'YAML.load_file(".github/workflows/governance-audit.yml"); puts "ok"'` passed.
2. `./scripts/verify-api-doc-commands.sh` passed.
3. `./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
4. `REPORT_JSON_PATH=/tmp/release-governance-diff-context.json ./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
5. `./scripts/verify-governance-trace-json.sh /tmp/release-governance-diff-context.json` passed.
6. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
7. `npm run check` fails locally due esbuild platform mismatch in current `node_modules`; CI `Quality Gate` remains authoritative.

## Risks and Mitigations

1. **Risk**: misconfigured retry/backoff values cause unstable behavior.
   - **Mitigation**: workflow validates bounded integer ranges before delivery attempts.
2. **Risk**: webhook spoofing concerns on receiver side.
   - **Mitigation**: optional HMAC signing with timestamped signature contract.
3. **Risk**: receiver incompatibility with signing headers.
   - **Mitigation**: HMAC signing remains opt-in and does not change baseline payload format.

## Decisions

1. Keep retry/backoff and signing controls optional and configuration-driven.
2. Use linear backoff to keep retry behavior simple and predictable.
3. Use `sha256=<hex>` signature format for broad receiver compatibility.

## Follow-Ups

1. Add optional jitter support to retry intervals to reduce synchronized retries.
2. Add optional replay-window guidance/examples for timestamp validation on webhook receivers.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
