# Sprint 044: Source Build Docs Corrections

- **Sprint ID**: `044`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README and deployment guide showed `cd 900invoice` after cloning `https://github.com/900Labs/900Invoice.git`. Git creates a `900Invoice` directory by default, so the lowercase command fails on case-sensitive filesystems. The same source-build docs also used `cargo tauri` commands without listing the Tauri CLI as a prerequisite.

## Goals

1. Make clone/build commands work on case-sensitive filesystems.
2. Document the Tauri CLI prerequisite for commands that use `cargo tauri`.
3. Keep README and deployment guide setup instructions aligned.

## In Scope

1. README source-build prerequisite and clone directory correction.
2. Deployment guide prerequisite and clone directory correction.
3. Changelog and sprint documentation.

## Out of Scope

1. Installing Tauri CLI in this repository.
2. Adding npm scripts or changing package dependencies.
3. Building platform installer artifacts.

## Deliverables

1. README uses `cd 900Invoice` after clone.
2. Deployment guide uses `cd 900Invoice` after clone.
3. Both docs list Tauri CLI v2 as required for `cargo tauri` commands.

## Validation

1. `cargo tauri --version` failed locally before the docs change, confirming the CLI is not an implicit prerequisite in this workspace.
2. `rg -n "cd 900invoice" README.md docs/DEPLOYMENT.md` returned no matches after the change.
3. `./scripts/verify-api-doc-commands.sh` passed.
4. `npm run check` passed.
5. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
6. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Users may expect `npm install` alone to provide the Tauri CLI.
   - **Mitigation**: The prerequisite now explicitly lists Tauri CLI v2 before the `cargo tauri` commands.
2. **Risk**: Case-insensitive local machines can hide clone-directory casing bugs.
   - **Mitigation**: README and deployment guide now use the exact default clone directory.

## Decisions

1. Correct documentation without changing package dependencies in this sprint.
2. Keep the documented command style as `cargo tauri` because the existing docs and release/deployment guides are written around the Rust CLI workflow.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
