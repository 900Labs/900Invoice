# Sprint 047: Deployment Release Parity

- **Sprint ID**: `047`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The deployment guide correctly documents the current source release model later in the file, but earlier sections still implied that CI builds macOS universal binaries and that auto-update is currently supported. The live release workflow only publishes a source tarball and checksums, and the repository does not include a Tauri updater plugin or updater config.

## Goals

1. Align deployment documentation with the current automated release workflow.
2. Make local platform build output distinct from published release artifacts.
3. Mark auto-update as future hardening instead of a currently enabled capability.

## In Scope

1. Deployment guide release-model caveat.
2. Production build output wording.
3. macOS CI binary wording.
4. Auto-update section wording.
5. Changelog and sprint documentation.

## Out of Scope

1. Adding platform binary build jobs.
2. Adding updater dependencies or Tauri updater configuration.
3. Changing release workflow behavior.

## Deliverables

1. `docs/DEPLOYMENT.md` states that current automated releases publish source archives and checksums.
2. `docs/DEPLOYMENT.md` frames bundle paths as local build outputs that vary by host/target.
3. The macOS section no longer says CI builds universal binaries.
4. The auto-update section no longer claims the app currently supports enabled updater delivery.

## Validation

1. `rg -n "CI/CD release workflow builds a universal binary|supports Tauri's built-in auto-update mechanism" docs/DEPLOYMENT.md` returned no matches.
2. `./scripts/verify-api-doc-commands.sh` passed.
3. `npm run check` passed.
4. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
5. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Users may read local build output examples as published release artifacts.
   - **Mitigation**: The guide now states that current automated releases publish source archives and checksums, and frames bundle outputs as local build examples.
2. **Risk**: Future auto-update work could need this section restored as active instructions.
   - **Mitigation**: The setup outline remains in place but is explicitly marked as future hardening.

## Decisions

1. Update documentation only; the release workflow remains source-archive based in this sprint.
2. Keep platform packaging examples because maintainers can still build those locally with Tauri.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
