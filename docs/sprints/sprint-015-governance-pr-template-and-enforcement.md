# Sprint 015: Governance PR Template and Checklist Enforcement

- **Sprint ID**: `015`
- **Date**: 2026-03-06
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 014 follow-ups required two controls:

1. Pull-request template confirmation for governance-impacting changes.
2. CI enforcement that governance-impacting sprint docs reference `docs/MAINTAINER_CHECKLIST.md`.

## Goals

1. Add maintainer governance confirmation to the PR template.
2. Fail governance-impacting PRs if sprint docs omit checklist reference.
3. Align contributor and sprint-process docs with the new enforcement behavior.

## In Scope

1. Update `.github/pull_request_template.md` with governance confirmation section.
2. Add governance checklist verification script for CI.
3. Integrate the new script into CI for pull requests.
4. Update process docs and changelog.

## Out of Scope

1. Runtime product behavior changes.
2. Branch protection rule changes.
3. External notification integrations.

## Deliverables

1. Updated `.github/pull_request_template.md`:
   - added `Maintainer Governance Confirmation (If Applicable)` section
   - includes explicit checks for governance-impacting PRs
2. Added `scripts/verify-governance-sprint-checklist.sh`:
   - detects governance-impacting file changes
   - requires at least one changed sprint doc in `docs/sprints/`
   - requires checklist reference (`docs/MAINTAINER_CHECKLIST.md`) in changed sprint docs
3. Updated `.github/workflows/ci.yml`:
   - added PR-only step to run governance checklist enforcement
4. Updated docs:
   - `docs/SPRINT_PROCESS.md`
   - `CONTRIBUTING.md`
   - `CHANGELOG.md`

## Validation

1. `bash -n scripts/verify-governance-sprint-checklist.sh` passed.
2. `./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
3. `./scripts/verify-api-doc-commands.sh` passed (`60` commands).
4. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
5. `npm run check` timed out in this local environment; CI `Quality Gate` remains required and authoritative.

## Risks and Mitigations

1. **Risk**: false positives on non-governance PRs.
   - **Mitigation**: enforcement script scopes to explicit governance-impacting file set.
2. **Risk**: governance PRs blocked when sprint docs are missing.
   - **Mitigation**: clear CI error messages and PR template guidance.

## Decisions

1. Enforce governance checklist reference in CI rather than manual review only.
2. Keep governance-impacting file detection explicit and maintainable via script allowlist.

## Follow-Ups

1. Extend enforcement to validate a checklist completion block directly in sprint docs if a stable format is adopted.
2. Add release-gate mirror check for governance checklist enforcement parity outside PR context.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.
