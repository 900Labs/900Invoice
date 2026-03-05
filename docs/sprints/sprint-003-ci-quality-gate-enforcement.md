# Sprint 003: CI Quality Gate Enforcement

- **Sprint ID**: `003`
- **Date**: 2026-03-05
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 002 defined a local quality gate, but it was not yet enforced by automation. Contributors could merge changes without guaranteed execution of the documented baseline checks.

## Goals

1. Enforce the quality gate automatically in CI.
2. Ensure CI behavior matches the documented command baseline.
3. Update contributor and sprint docs so CI expectations are explicit.

## In Scope

1. Add a GitHub Actions workflow for quality-gate enforcement.
2. Update process docs to reference CI enforcement.
3. Add a sprint record and changelog update.

## Out of Scope

1. Functional product feature changes.
2. Test suite expansion.
3. Release packaging changes.

## Deliverables

1. Added `.github/workflows/ci.yml`:
   - triggers on PRs to `main` and pushes to `main`
   - runs `npm install`, `npm run check`, `cargo check`, `cargo test`, and `cargo clippy -D warnings`
   - sets `CARGO_TARGET_DIR=/tmp/900invoice-target`
   - installs required Linux/Tauri build dependencies
   - enables workflow concurrency cancellation for superseded runs
2. Updated `docs/QUALITY_GATE.md` with CI enforcement section and workflow path.
3. Updated `docs/SPRINT_PROCESS.md` definition-of-done to require passing CI quality-gate checks for `main` PRs.
4. Updated `CONTRIBUTING.md` to explicitly reference CI quality-gate enforcement.
5. Updated `CHANGELOG.md` with the new CI enforcement entry.

## Validation

1. `npm run check` passed.
2. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo check --manifest-path src-tauri/Cargo.toml` passed.
3. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml` passed (`52` tests).
4. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.

## Risks and Mitigations

1. **Risk**: CI and docs drifting apart.
   - **Mitigation**: CI workflow mirrors the same command sequence documented in `docs/QUALITY_GATE.md`.
2. **Risk**: Rust job lock contention in automation.
   - **Mitigation**: workflow uses a dedicated `CARGO_TARGET_DIR` consistent with local guidance.

## Decisions

1. Keep one quality-gate CI job with sequential checks for clear pass/fail diagnostics.
2. Enforce on both `pull_request` and `push` to `main` for merge-path coverage.

## Follow-Ups

1. Add branch protection requiring the `Quality Gate` check before merge.
2. Add optional matrix job(s) for cross-platform compile confidence (macOS/Windows).
3. Add markdown/link lint checks in a separate docs-quality workflow.
