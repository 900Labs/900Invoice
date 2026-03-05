# Sprint 006: Release Gate Foundation

- **Sprint ID**: `006`
- **Date**: 2026-03-05
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Deployment docs referenced a release workflow that did not exist in the repository. Release automation claims and implementation were out of sync.

## Goals

1. Add a concrete release-gate workflow for tagged releases.
2. Publish verifiable release artifacts with checksums.
3. Align deployment/release documentation with current automation behavior.

## In Scope

1. Add `.github/workflows/release.yml`.
2. Add `docs/RELEASE.md` runbook.
3. Update release/deployment references in docs.
4. Add sprint/changelog updates.

## Out of Scope

1. Full cross-platform signed binary pipeline.
2. Notarization and code-signing execution.
3. Package-manager publishing automation.

## Deliverables

1. Added `.github/workflows/release.yml`:
   - triggers on `push` tags `v*`
   - supports `workflow_dispatch` for manual release-gate runs
   - runs full quality gate (`verify-api-doc-commands`, `npm check`, `cargo check/test/clippy`)
   - builds source tarball (`900invoice-<version>-src.tar.gz`)
   - generates `SHA256SUMS.txt`
   - uploads artifacts and publishes GitHub release assets for tag runs
2. Added `docs/RELEASE.md` with:
   - pre-tag checklist
   - tagged release flow
   - manual release-gate flow
   - post-release checklist
   - future hardening roadmap
3. Updated `docs/DEPLOYMENT.md`:
   - aligned release automation claims to current workflow behavior
   - removed references to non-existent workflows
   - clarified secrets as current vs optional future signing pipeline
4. Updated `README.md` documentation index with `docs/RELEASE.md`.
5. Updated `CHANGELOG.md` with release-gate/runbook additions.

## Validation

1. `./scripts/verify-api-doc-commands.sh` passed.
2. `npm run check` passed (`0` errors, `0` warnings).
3. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo check --manifest-path src-tauri/Cargo.toml` passed.
4. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml` passed.
5. `CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.

## Risks and Mitigations

1. **Risk**: release expectations exceed current automation capabilities.
   - **Mitigation**: explicit runbook and deployment docs now separate current automation from future signing/build goals.
2. **Risk**: tagged release without validation.
   - **Mitigation**: release workflow executes the full quality gate before publishing assets.

## Decisions

1. Start with source artifact + checksum publication as a reliable baseline.
2. Keep signed cross-platform packaging as a planned follow-up rather than shipping partial signing now.

## Follow-Ups

1. Add build matrix for Linux/macOS/Windows bundles.
2. Integrate code signing and notarization workflows.
3. Generate and publish SBOM/provenance artifacts.
