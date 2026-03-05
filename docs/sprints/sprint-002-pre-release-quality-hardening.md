# Sprint 002: Pre-Release Quality Hardening

- **Sprint ID**: `002`
- **Date**: 2026-03-05
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Before public open-source release, the codebase required a focused quality control pass to reduce security risk, enforce lifecycle invariants, and stabilize contributor workflow.

## Goals

1. Eliminate high-risk input and file-handling paths.
2. Enforce invoice and payment state invariants across commands.
3. Raise pre-merge quality gate rigor and documentation quality.
4. Ensure repository links, artifacts, and generated outputs are release-ready.

## In Scope

1. Rust command/service hardening in invoice, payment, import/export, and PDF flows.
2. Frontend check hygiene for invoice editor warnings.
3. Contributor and sprint process documentation hardening.
4. Repository hygiene for generated/local-only artifacts.

## Out of Scope

1. New user-facing product features.
2. Database schema redesign.
3. CI pipeline redesign.
4. Multi-device sync implementation.

## Deliverables

1. Security hardening for business logo ingestion and PDF logo read paths.
2. CSV export formula-injection mitigation.
3. Draft-only mutation guardrails for invoice and line-item edits.
4. Payment validation hardening and status rollback correction when deleting payments.
5. Tauri config cleanup (`app.title` removal) to satisfy strict Rust/Tauri validation.
6. Lockfile introduction for deterministic installs (`package-lock.json`, `src-tauri/Cargo.lock`).
7. Generated artifact hygiene (`src-tauri/gen/` ignored) and local scope hygiene (`/pdf/` remains local only).
8. Contributor-facing quality runbook at `docs/QUALITY_GATE.md`.
9. Sprint/process doc updates with isolated-workspace parallel automation rules.
10. Repository slug/link corrections to `900Labs/900Invoice`.

## Validation

1. `npm run check` passed.
2. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo check --manifest-path src-tauri/Cargo.toml` passed.
3. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml` passed (`52` tests).
4. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.

## Risks and Mitigations

1. **Risk**: restore-related duplicate files and git metadata artifacts.
   - **Mitigation**: removed duplicate `* 2*` files and restored single-checkout integration workflow.
2. **Risk**: lock contention from concurrent Rust jobs.
   - **Mitigation**: standardized `CARGO_TARGET_DIR=/tmp/900invoice-target` for quality gate commands.
3. **Risk**: contributors using outdated repo links.
   - **Mitigation**: updated docs and changelog links to the canonical `900Labs/900Invoice` slug.

## Decisions

1. Keep one integration checkout for all final git operations.
2. Run parallel automation only in isolated agent workspaces with disjoint write scopes.
3. Require `docs/QUALITY_GATE.md` as the default validation baseline for sprint PRs.

## Follow-Ups

1. Add CI enforcement for `docs/QUALITY_GATE.md` command set.
2. Add focused tests for restore-path validation and CSV parser robustness.
3. Add regression tests for payment status rollback edge cases across partial-payment scenarios.
