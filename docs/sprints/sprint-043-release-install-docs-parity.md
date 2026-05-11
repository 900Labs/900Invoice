# Sprint 043: Release Install Docs Parity

- **Sprint ID**: `043`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README installation section advertised downloadable Windows, macOS, and Linux platform binaries. The current release runbook and release workflow only publish source archives and checksums on tagged releases; signed platform installers are explicitly listed as future hardening.

## Goals

1. Remove the README claim that platform installers are already published.
2. Point users at the current source-build path until binary release automation exists.
3. Keep release documentation consistent with `docs/RELEASE.md`.

## In Scope

1. README installation wording.
2. Changelog entry.
3. Sprint documentation.

## Out of Scope

1. Adding cross-platform release build automation.
2. Creating or publishing a GitHub release.
3. Code signing, notarization, or package-manager publishing.

## Deliverables

1. README now describes tagged releases as source archives/checksums under the current workflow.
2. README no longer lists unpublished `.msi`, `.exe`, `.dmg`, `.AppImage`, `.deb`, or `.rpm` binaries as available downloads.
3. Changelog and sprint record document the correction.

## Validation

1. `gh release list --repo 900Labs/900Invoice --limit 10` returned no published releases.
2. `.github/workflows/release.yml` review confirmed the workflow publishes source archives and checksums.
3. `docs/RELEASE.md` review confirmed platform-specific binary bundles are not yet automated.
4. `./scripts/verify-api-doc-commands.sh` passed.
5. `npm run check` passed.
6. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
7. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Users may expect one-click installers from the README.
   - **Mitigation**: README now states the current release model and directs users to build from source until platform binaries are published.
2. **Risk**: The README could drift from release automation again.
   - **Mitigation**: The wording references the current release workflow and release runbook rather than hardcoding unavailable artifact names.

## Decisions

1. Correct the user-facing README instead of adding release automation in this sprint.
2. Keep detailed platform packaging instructions in deployment/release docs.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
