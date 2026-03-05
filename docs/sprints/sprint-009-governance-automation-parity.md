# Sprint 009: Governance Automation Parity

- **Sprint ID**: `009`
- **Date**: 2026-03-05
- **Status**: Completed
- **Owner**: 900 Labs

## Context

After strict branch protection was activated, autonomous sprint merging deadlocked because GitHub does not allow authors to self-approve pull requests while policy required one approving review.

## Goals

1. Align governance automation with the active maintainer workflow.
2. Keep strict status-check enforcement while removing self-approval deadlocks.
3. Document policy defaults and override mechanisms clearly for open-source contributors.

## In Scope

1. Update policy apply script to support configurable review requirements.
2. Update policy verification script to assert exact configured review count.
3. Update branch-protection and contributing docs to match enforced policy.
4. Record the governance adjustment in changelog.

## Out of Scope

1. Product feature changes.
2. Runtime business logic changes.
3. Relaxing required status checks or linear-history requirements.

## Deliverables

1. Updated `scripts/apply-repo-policy.sh`:
   - added `REQUIRED_APPROVING_REVIEW_COUNT` (default `0`)
   - validates value is a non-negative integer
   - applies the configured review count in branch protection payload
2. Updated `scripts/verify-repo-policy.sh`:
   - added `REQUIRED_APPROVING_REVIEW_COUNT` (default `0`)
   - validates configuration input
   - verifies exact expected review count instead of `>= 1`
   - replaced `rg` usage with `grep` for minimal-runner compatibility
3. Updated `docs/BRANCH_PROTECTION.md` with default review-count policy and override examples.
4. Updated `CONTRIBUTING.md` code-review section to reflect current branch-protection behavior.
5. Updated `CHANGELOG.md` in `[Unreleased]`.
6. Removed `rg` dependency from governance scripts (`apply` + `verify`).

## Validation

1. `bash -n scripts/apply-repo-policy.sh scripts/verify-repo-policy.sh` passed.
2. `./scripts/verify-api-doc-commands.sh` passed (`60` commands).
3. `npm run check` passed (`0` errors, `0` warnings).
4. `STRICT=1 ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.

## Risks and Mitigations

1. **Risk**: perception that no required approvals lowers quality.
   - **Mitigation**: keep strict `Quality Gate`, conversation resolution, and linear history; maintain documented recommendation for maintainer review.
2. **Risk**: environment drift if teams later require approvals.
   - **Mitigation**: policy scripts now support explicit override (`REQUIRED_APPROVING_REVIEW_COUNT=1` or higher).

## Decisions

1. Default governance baseline is `required_approving_review_count = 0` for autonomous maintainer operation.
2. Status-check strictness remains mandatory (`Quality Gate`, strict branch checks, linear history, conversation resolution).

## Follow-Ups

1. Add optional scheduled governance audit workflow to detect and alert on policy drift.
2. Define team-scale profile documentation (`solo`, `small-team`, `enterprise`) using policy-script environment overrides.
