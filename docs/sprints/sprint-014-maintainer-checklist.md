# Sprint 014: Maintainer Checklist Template

- **Sprint ID**: `014`
- **Date**: 2026-03-06
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 013 closed governance profile onboarding and audit assertion follow-ups, but a maintainer-ready checklist template was still missing for repeatable governance profile selection and variable verification.

## Goals

1. Add a maintainer checklist template that standardizes governance/profile operations.
2. Link the checklist from core contributor, sprint, release, and governance docs.
3. Keep the checklist aligned with existing profile-based policy automation.

## In Scope

1. Create `docs/MAINTAINER_CHECKLIST.md`.
2. Link checklist in `README.md`, `CONTRIBUTING.md`, `docs/SPRINT_PROCESS.md`, `docs/RELEASE.md`, and `docs/GOVERNANCE_AUDIT.md`.
3. Update changelog and sprint records.

## Out of Scope

1. Runtime product behavior changes.
2. Branch protection policy redesign.
3. External alert integrations.

## Deliverables

1. Added `docs/MAINTAINER_CHECKLIST.md` including:
   - governance profile selection matrix (`solo`, `small-team`, `enterprise`)
   - apply/verify policy checklist
   - sprint merge hygiene checklist
   - governance audit triage checklist
   - release readiness checklist
2. Updated docs links and references:
   - `README.md`
   - `CONTRIBUTING.md`
   - `docs/SPRINT_PROCESS.md`
   - `docs/RELEASE.md`
   - `docs/GOVERNANCE_AUDIT.md`
3. Updated `CHANGELOG.md` in `[Unreleased]`.

## Validation

1. `./scripts/verify-api-doc-commands.sh` passed (`60` commands).
2. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
3. `npm run check` timed out in this local environment; CI `Quality Gate` remains required and authoritative.

## Risks and Mitigations

1. **Risk**: checklist and policy scripts diverge over time.
   - **Mitigation**: linked checklist directly to existing scripts and profile contract docs.
2. **Risk**: maintainers skip governance checks during fast merges.
   - **Mitigation**: inserted checklist references into sprint/release/governance runbooks.

## Decisions

1. Keep one central maintainer checklist file instead of scattering checklists across multiple docs.
2. Treat governance profile selection as first-class maintainer setup, not implicit behavior.

## Follow-Ups

1. Add a pull-request template section for maintainers to confirm checklist completion on governance-impacting changes.
2. Add automation to fail governance PRs when checklist reference is missing from sprint docs.
