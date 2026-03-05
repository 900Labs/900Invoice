# Sprint 010: Governance Drift Audit

- **Sprint ID**: `010`
- **Date**: 2026-03-05
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 009 identified scheduled governance auditing as the next control to reduce policy drift risk between documented standards and live GitHub settings.

## Goals

1. Add low-overhead automated governance drift detection.
2. Keep verification strict while preserving manual override controls.
3. Document operation and incident response clearly for global contributors.

## In Scope

1. Add a scheduled/manual GitHub Actions governance audit workflow.
2. Add governance audit documentation.
3. Link governance audit documentation from primary project docs.
4. Update changelog and sprint record.

## Out of Scope

1. Runtime product behavior changes.
2. Branch protection redesign.
3. Additional release artifact changes.

## Deliverables

1. Added `.github/workflows/governance-audit.yml`:
   - weekly schedule (`03:17 UTC` every Monday)
   - `workflow_dispatch` trigger
   - strict repository-policy verification
   - API docs parity verification
2. Added `docs/GOVERNANCE_AUDIT.md` runbook:
   - purpose, triggers, token behavior, manual run guidance, failure response
3. Updated `README.md` documentation index with Governance Audit link.
4. Updated `docs/BRANCH_PROTECTION.md` operational guidance to keep governance audit enabled.
5. Updated `CHANGELOG.md` in `[Unreleased]`.

## Validation

1. `./scripts/verify-api-doc-commands.sh` passed (`60` commands).
2. `npm run check` passed (`0` errors, `0` warnings).
3. `STRICT=1 ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.

## Risks and Mitigations

1. **Risk**: GitHub token permission variance for branch-protection API access.
   - **Mitigation**: workflow supports `GH_ADMIN_TOKEN` secret override and documents fallback behavior.
2. **Risk**: unnoticed settings drift between releases.
   - **Mitigation**: weekly scheduled audit plus manual dispatch for immediate verification.

## Decisions

1. Keep governance audits lightweight (no Rust/Node toolchain setup required).
2. Reuse existing verification scripts to avoid split governance logic.

## Follow-Ups

1. Add issue/notification routing for governance audit failures.
2. Define profile presets (`solo`, `small-team`, `enterprise`) in governance docs with explicit environment-variable matrices.
