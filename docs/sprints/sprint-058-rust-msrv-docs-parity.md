# Sprint 058: Rust MSRV Docs Parity

- **Sprint ID**: `058`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README and deployment guide advertised Rust 1.75+ as sufficient for source builds, but the locked dependency graph includes crates that declare Rust 1.88.0. That made the published source-build prerequisites lower than the actual build requirement.

## Goals

1. Document the Rust source-build prerequisite that matches the locked dependency graph.
2. Pin the crate package metadata so Cargo exposes the intended project MSRV.
3. Keep source-build and deployment guidance consistent.

## In Scope

1. README Rust badge and source-build prerequisite.
2. Deployment guide Rust prerequisite.
3. `src-tauri/Cargo.toml` package metadata.
4. Changelog and sprint documentation.

## Out of Scope

1. Downgrading dependencies to support older Rust compilers.
2. Changing Node.js or Tauri CLI prerequisites.
3. Release binary packaging.

## Deliverables

1. README and deployment docs advertise Rust 1.88+.
2. `src-tauri/Cargo.toml` declares `rust-version = "1.88"`.
3. Validation records the locked dependency graph MSRV.

## Validation

1. `rg -n -F 'Rust 1.75+' README.md docs/DEPLOYMENT.md` returned no matches.
2. `rg -n -F -e 'Rust-1.88+' -e 'Rust 1.88+' README.md docs/DEPLOYMENT.md` confirmed the updated docs.
3. `cargo metadata --manifest-path src-tauri/Cargo.toml --format-version 1 --locked --quiet` confirmed the maximum locked dependency `rust_version` is `1.88.0`.
4. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo check --manifest-path src-tauri/Cargo.toml` passed.
5. `./scripts/verify-api-doc-commands.sh` passed.
6. `npm run check` passed.
7. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
8. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Contributors on older Rust toolchains may only discover the requirement after reading the docs.
   - **Mitigation**: Cargo now also declares `rust-version = "1.88"`, making the requirement machine-readable.
2. **Risk**: Future dependency updates may raise the effective MSRV again.
   - **Mitigation**: Validation records the metadata check used to compare docs with the locked dependency graph.

## Decisions

1. Raise the documented prerequisite instead of downgrading the dependency graph.
2. Use Rust 1.88 as the project package MSRV because locked dependencies already require 1.88.0.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
