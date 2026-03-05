# Sprint 008: Governance Strict Activation

- **Sprint ID**: `008`
- **Date**: 2026-03-05
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 007 delivered branch-protection readiness and repository merge-policy enforcement, but branch protection could not be applied while the repository was private under plan restrictions.

The repository is now public, so branch protection can be fully activated and strict policy verification can be enforced.

## Goals

1. Activate branch protection on `main`.
2. Enforce strict repository-policy verification in release governance.
3. Update governance docs to reflect active enforcement state.

## In Scope

1. Apply and verify repository + branch protection policy.
2. Make strict policy verification default behavior.
3. Add release workflow check for strict policy verification.
4. Update sprint/changelog/governance docs.

## Out of Scope

1. Changes to product runtime behavior.
2. New release artifact formats.
3. Branch-protection policy redesign.

## Deliverables

1. Applied branch protection on `main` via `scripts/apply-repo-policy.sh`.
2. Verified branch protection in strict mode via `STRICT=1 ./scripts/verify-repo-policy.sh`.
3. Updated `scripts/verify-repo-policy.sh`:
   - default mode is now strict (`STRICT=1` by default)
4. Updated `.github/workflows/release.yml`:
   - added strict repository-policy verification step before release-gate checks
5. Updated governance docs:
   - `docs/BRANCH_PROTECTION.md` status and strict-mode guidance
   - `docs/RELEASE.md` release checklist strict-policy requirement
6. Updated `CHANGELOG.md`.

## Validation

1. `./scripts/apply-repo-policy.sh 900Labs/900Invoice main` passed.
2. `STRICT=1 ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
3. `./scripts/verify-api-doc-commands.sh` passed (`60` commands).
4. `npm run check` passed (`0` errors, `0` warnings).
5. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo check --manifest-path src-tauri/Cargo.toml` passed.
6. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml` passed.
7. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.

## Risks and Mitigations

1. **Risk**: governance drift from future admin setting changes.
   - **Mitigation**: strict verification is now default and included in release workflow.
2. **Risk**: accidental relaxation of branch safeguards.
   - **Mitigation**: policy remains codified in scripts + runbook and validated before releases.

## Decisions

1. Treat strict policy verification as default behavior, not optional.
2. Gate releases on repository-policy compliance.

## Follow-Ups

1. Add scheduled policy-audit workflow (nightly/weekly) with alerting on drift.
2. Add repo-ruleset evaluation for broader governance controls as needed.
