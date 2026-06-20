# Sprint 068 - Node 24 GitHub Actions Cleanup

Status: Completed
Date: 2026-06-20
Owner: 900 Labs

## Context

The `v1.0.0` release and CI runs completed successfully, but GitHub emitted deprecation warnings for Node.js 20-based Actions. The warning affected workflow actions used by CI, Release Gate, governance audit, and governance artifact inventory jobs.

Source builds still support the package engine range documented in the repository: Node.js `20.19+`, `22.12+`, or `24+`. This sprint only updates the GitHub Actions runtime path so hosted automation runs on Node 24-compatible action majors.

## Goals

1. Remove Node.js 20 action-runtime deprecation warnings from GitHub Actions.
2. Keep CI and Release Gate validation behavior unchanged.
3. Update governance workflows that use GitHub JavaScript actions.
4. Keep the source-build Node engine range unchanged.

## Deliverables

1. Updated CI checkout/setup actions to `actions/checkout@v7` and `actions/setup-node@v6`.
2. Updated CI and Release Gate frontend validation to run on Node.js `24`.
3. Updated Release Gate artifact and release actions to `actions/upload-artifact@v7` and `softprops/action-gh-release@v3`.
4. Updated governance audit and inventory workflows to Node 24-compatible action majors:
   - `actions/checkout@v7`
   - `actions/upload-artifact@v7`
   - `actions/github-script@v9`

## Validation

Run from the repository root:

```bash
./scripts/verify-api-doc-commands.sh
npm run check
CARGO_TARGET_DIR=/tmp/900invoice-sprint068-target cargo check --manifest-path src-tauri/Cargo.toml
CARGO_TARGET_DIR=/tmp/900invoice-sprint068-target cargo test --manifest-path src-tauri/Cargo.toml
CARGO_TARGET_DIR=/tmp/900invoice-sprint068-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
git diff --check
```

Expected result: local quality gates pass; PR CI validates the upgraded GitHub Actions majors without Node.js 20 runtime deprecation warnings.

## Decisions

1. Keep local contributor Node.js support at the existing package-engine range.
2. Prefer stable major-version upgrades over setting compatibility override environment variables.
3. Keep Release Gate source-archive behavior unchanged; this sprint only changes workflow runtime dependencies.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile selected or explicitly unchanged: `solo`.
- [x] Branch protection expectations reviewed against `docs/BRANCH_PROTECTION.md`.
- [x] Maintainer handoff expectations reviewed against `docs/MAINTAINER_CHECKLIST.md`.
- [x] Release or validation impact documented in this sprint record.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
