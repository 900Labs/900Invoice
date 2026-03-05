# Sprint 004: Security Regression Tests

- **Sprint ID**: `004`
- **Date**: 2026-03-05
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 002 introduced security hardening for CSV export sanitization and logo file handling. Those protections needed direct regression tests to prevent future regressions during refactors.

## Goals

1. Add focused tests for CSV formula-injection mitigation.
2. Add focused tests for hardened logo loading constraints.
3. Keep tests deterministic and compatible with the existing quality gate.

## In Scope

1. Unit tests in `src-tauri/src/commands/import_export.rs`.
2. Unit tests in `src-tauri/src/services/pdf_engine.rs`.
3. Sprint/changelog documentation updates.

## Out of Scope

1. Behavioral changes to command/service runtime logic.
2. Integration tests requiring Tauri runtime bootstrapping.
3. Backup/restore redesign.

## Deliverables

1. Added CSV regression tests covering:
   - formula-like prefix neutralization (`=`, `+`, `-`, `@`, tab, carriage return)
   - quoting behavior after sanitization
   - safe value pass-through behavior
   - basic quoted-field parse behavior
2. Added PDF logo loading regression tests covering:
   - unsupported extension rejection
   - >2MB file rejection
   - allowed extension and size acceptance
3. Updated `CHANGELOG.md` with security test coverage addition.

## Validation

1. `npm run check` passed (`0` errors, `0` warnings).
2. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo check --manifest-path src-tauri/Cargo.toml` passed.
3. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml` passed.
4. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.

## Risks and Mitigations

1. **Risk**: security hardening regressions reintroduced by future refactors.
   - **Mitigation**: direct unit-test assertions on critical sanitization and file-validation paths.
2. **Risk**: test flakiness from filesystem usage.
   - **Mitigation**: tests use temporary files with unique names and explicit cleanup.

## Decisions

1. Prioritized deterministic unit tests over runtime-coupled command integration tests.
2. Kept coverage centered on high-risk security invariants first.

## Follow-Ups

1. Add restore-path validation tests for malformed backup payloads.
2. Add invoice/payment lifecycle integration tests over an isolated in-memory SQLite DB.
