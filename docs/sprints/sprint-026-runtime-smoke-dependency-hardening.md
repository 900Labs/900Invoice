# Sprint 026: Runtime Smoke Dependency Hardening

- **Sprint ID**: `026`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 025 added the runtime smoke runbook and script, but a fresh dependency install exposed a broken `aria-query@5.3.1` package payload requested by `svelte@5.53.7`. The missing generated role files caused Svelte compiler imports, `npm run check`, `npm run build`, and the runtime smoke script to fail.

## Goals

1. Make frontend dependency installs reproducible for runtime smoke checks.
2. Keep generated governance trace artifacts from dirtying local work trees.
3. Preserve governance sprint-documentation evidence for the merged Sprint 025 record.

## In Scope

1. Add an npm override for `aria-query@5.3.2`.
2. Refresh `package-lock.json` for the override.
3. Ignore local `.tmp/` governance reports.
4. Update changelog and sprint records with validation evidence.

## Out of Scope

1. Product feature changes.
2. API or database schema changes.
3. Runtime smoke script behavior changes.

## Deliverables

1. `package.json` override for `aria-query@5.3.2`.
2. `package-lock.json` lockfile update.
3. `.gitignore` entry for `.tmp/`.
4. Sprint documentation and changelog updates.

## Validation

1. `./scripts/verify-api-doc-commands.sh` passed.
2. `npm run check` passed.
3. `npm run build` passed.
4. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo check --manifest-path src-tauri/Cargo.toml` passed.
5. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml` passed with 59 tests.
6. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.
7. `INSTALL_NODE_DEPS=0 SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
8. `INSTALL_NODE_DEPS=1 SMOKE_PROFILE=baseline LEGACY_HARDWARE=1 CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
9. `REPORT_JSON_PATH=/tmp/sprint-025-governance-diff-context.json ./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
10. `./scripts/verify-governance-trace-json.sh /tmp/sprint-025-governance-diff-context.json` passed.

## Risks and Mitigations

1. **Risk**: npm cache or cloud-sync artifacts can leave partial package directories in `node_modules`.
   - **Mitigation**: verified fresh `npm ci` through the legacy baseline smoke path.
2. **Risk**: generated governance trace files can appear as untracked local changes.
   - **Mitigation**: ignored `.tmp/` at the repository level.

## Decisions

1. Use an npm override instead of a direct dev dependency because `aria-query` is a transitive Svelte compiler dependency.
2. Keep this as a follow-up sprint because Sprint 025 had already been squash-merged.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
