# Sprint 020: Checklist Completion Block Enforcement

- **Sprint ID**: `020`
- **Date**: 2026-03-07
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 015 introduced governance sprint-doc checklist reference enforcement and explicitly deferred a stronger follow-up: validate a structured checklist completion block directly in sprint docs once a stable format was defined.

## Goals

1. Define a stable, machine-parseable sprint-doc checklist completion block format.
2. Enforce that format for governance-impacting sprint docs.
3. Keep enforcement lightweight for minimal CI/runtime environments.

## In Scope

1. Extend `scripts/verify-governance-sprint-checklist.sh` with checklist completion block validation.
2. Add report output fields for block coverage/results in text and JSON outputs.
3. Update maintainer and sprint process documentation with the canonical block template.
4. Update changelog and sprint records.

## Out of Scope

1. Provider-specific incident notification logic.
2. Product runtime behavior changes.
3. Historical backfill of older sprint docs not touched by governance-impacting diffs.

## Deliverables

1. Updated `scripts/verify-governance-sprint-checklist.sh`:
   - validates `MAINTAINER_CHECKLIST_COMPLETION` block markers in changed sprint docs
   - requires at least three checked `- [x]` items inside the block
   - rejects unchecked `- [ ]` items inside the block
   - emits completion-block coverage lists in report text and JSON output
2. Updated `docs/SPRINT_PROCESS.md` with the required completion-block contract.
3. Updated `docs/MAINTAINER_CHECKLIST.md` with sprint-doc completion-block template.
4. Updated `CHANGELOG.md`.

## Validation

1. `bash -n scripts/verify-governance-sprint-checklist.sh` passed.
2. `./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
3. `REPORT_PATH=/tmp/governance-checklist-report.txt REPORT_JSON_PATH=/tmp/governance-checklist-report.json ./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
4. `./scripts/verify-api-doc-commands.sh` passed.
5. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
6. `npm run check` remains environment-sensitive locally; CI `Quality Gate` remains required and authoritative.

## Risks and Mitigations

1. **Risk**: contributors use inconsistent checklist block formats.
   - **Mitigation**: explicit marker contract and template in maintainer/sprint docs.
2. **Risk**: strict parsing causes false failures.
   - **Mitigation**: marker-based format with simple checked-item rules and clear error output.

## Decisions

1. Use explicit HTML-style marker lines for deterministic parsing in shell.
2. Require checked items only in the completion block to keep handoff state unambiguous.
3. Scope enforcement to changed sprint docs in governance-impacting diffs.

## Follow-Ups

1. Optionally add CI lint for duplicate or nested completion blocks in a single sprint doc.
2. Optionally add richer structured fields (owner/date/profile) inside the completion block when needed.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
