# Sprint 005: API Docs Parity Enforcement

- **Sprint ID**: `005`
- **Date**: 2026-03-05
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The previous API documentation had significant drift from the actual Tauri command surface. Several documented command names did not exist, and multiple live commands were missing.

## Goals

1. Restore `docs/API.md` parity with the current command registry in `src-tauri/src/lib.rs`.
2. Add an automated guardrail so command/documentation drift is caught in CI.
3. Update contributor quality-gate docs to include parity verification.

## In Scope

1. Rewrite `docs/API.md` using implementation-accurate command names and argument contracts.
2. Add a docs parity verification script.
3. Wire parity verification into CI and quality-gate docs.
4. Record sprint/changelog updates.

## Out of Scope

1. Runtime command behavior changes.
2. New product features.
3. API versioning redesign.

## Deliverables

1. Replaced stale API reference with implementation-accurate content in `docs/API.md`.
2. Added canonical command catalog block with explicit boundaries:
   - `<!-- COMMAND_CATALOG_START -->`
   - `<!-- COMMAND_CATALOG_END -->`
3. Added `scripts/verify-api-doc-commands.sh` to compare:
   - command names in `src-tauri/src/lib.rs`
   - command catalog entries in `docs/API.md`
4. Updated `.github/workflows/ci.yml` to run API docs parity verification.
5. Updated `docs/QUALITY_GATE.md` and `CONTRIBUTING.md` with parity-check command.
6. Updated `CHANGELOG.md` with parity and enforcement entries.

## Validation

1. `./scripts/verify-api-doc-commands.sh` passed (`60` commands in sync).
2. `npm run check` passed (`0` errors, `0` warnings).
3. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo check --manifest-path src-tauri/Cargo.toml` passed.
4. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml` passed.
5. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.

## Risks and Mitigations

1. **Risk**: future command additions not reflected in docs.
   - **Mitigation**: CI now fails on catalog mismatch.
2. **Risk**: false positives from malformed catalog block formatting.
   - **Mitigation**: parser requires strict bullet format and checks for duplicates.

## Decisions

1. Keep parity check lightweight and shell-native to run in local and CI environments.
2. Treat command name parity as a mandatory quality gate, not optional lint.

## Follow-Ups

1. Add schema-level validation for command argument/return contracts beyond command names.
2. Introduce API changelog sectioning by command module for release notes.
