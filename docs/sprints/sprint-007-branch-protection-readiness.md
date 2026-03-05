# Sprint 007: Branch Protection Readiness

- **Sprint ID**: `007`
- **Date**: 2026-03-05
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Branch protection is required for open-source governance quality, but the repository is currently private and GitHub returned a plan/visibility restriction when applying branch protection via API.

## Goals

1. Enforce squash-only merge policy where immediately available.
2. Add executable automation to apply and verify branch-protection policy as soon as repository visibility allows it.
3. Document branch protection requirements and operating procedures.

## In Scope

1. Repository merge policy enforcement.
2. Branch protection apply/verify scripts.
3. Branch protection and release governance documentation.
4. Sprint/changelog updates.

## Out of Scope

1. Changing repository visibility.
2. Paid plan upgrades.
3. CI-enforced strict branch-protection verification (deferred until protection is available).

## Deliverables

1. Applied repository merge policy (live settings):
   - `allow_squash_merge=true`
   - `allow_merge_commit=false`
   - `allow_rebase_merge=false`
   - `delete_branch_on_merge=true`
2. Added `scripts/apply-repo-policy.sh`:
   - applies merge settings
   - attempts branch protection on `main`
   - handles current private-repo plan restriction with explicit notice
3. Added `scripts/verify-repo-policy.sh`:
   - validates merge policy
   - validates branch-protection rules when available
   - supports `STRICT=1` to fail when branch protection is unavailable
4. Added `docs/BRANCH_PROTECTION.md` with policy targets and operations.
5. Updated release/sprint/docs index references:
   - `README.md`
   - `docs/SPRINT_PROCESS.md`
   - `docs/RELEASE.md`
6. Updated `CHANGELOG.md`.

## Validation

1. `./scripts/apply-repo-policy.sh 900Labs/900Invoice main` applied merge settings and reported branch-protection availability notice.
2. `./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed (with branch-protection availability notice).
3. `./scripts/verify-api-doc-commands.sh` passed (`60` commands).
4. `npm run check` passed (`0` errors, `0` warnings).
5. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo check --manifest-path src-tauri/Cargo.toml` passed.
6. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml` passed.
7. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.

## Risks and Mitigations

1. **Risk**: branch protection not enforced while repository remains private under current plan restrictions.
   - **Mitigation**: scripts and runbook are ready; re-run apply script immediately after repository becomes public.
2. **Risk**: governance drift after admin setting changes.
   - **Mitigation**: verification script supports routine audits and strict mode.

## Decisions

1. Treat merge-policy enforcement and branch-protection readiness as separate tracks until branch protection is platform-available.
2. Preserve strict policy definitions in docs even when temporary platform constraints block enforcement.

## Follow-Ups

1. When repository becomes public, run:
   - `./scripts/apply-repo-policy.sh 900Labs/900Invoice main`
   - `STRICT=1 ./scripts/verify-repo-policy.sh 900Labs/900Invoice main`
2. Add periodic governance audit job once branch protection is available.
