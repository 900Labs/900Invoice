# Sprint 063: Public Release Docs Cleanup

- **Sprint ID**: `063`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Before making the repository public, active docs needed a privacy and readability pass. The tracked-file scan found no personal local paths or tracked operating-system metadata files, but `docs/DEPLOYMENT.md` used a named Windows profile path example. The README documentation section was also a long flat list, making it harder for new GitHub readers to find build, product, maintainer, and release docs.

## Goals

1. Remove named-user local path examples from active public docs.
2. Add a repeatable public-release privacy and documentation checklist.
3. Reorganize the README documentation map for human readers.
4. Wire public-readiness checks into release documentation.

## In Scope

1. `README.md` documentation map.
2. `docs/DEPLOYMENT.md` Windows data-location example.
3. New `docs/PUBLIC_RELEASE.md`.
4. `docs/RELEASE.md` pre-tag checklist.
5. Changelog and sprint documentation.

## Out of Scope

1. Changing application behavior.
2. Removing ignored local workspace artifacts that are not tracked by Git.
3. Changing release automation.

## Deliverables

1. README documentation links are grouped for build/use, product/architecture, and maintainer/contributor readers.
2. Deployment docs no longer include a named-user Windows path example.
3. Public release checklist documents privacy scans, public-facing files, stale-doc checks, validation, and GitHub readiness.
4. Release runbook references the public release checklist for visibility changes and public release prep.

## Validation

1. `git ls-files | rg '(^|/)\.DS_Store$|(^|/)Thumbs\.db$|(^|/)desktop\.ini$'` returned no matches.
2. A tracked-file privacy scan for local home-directory paths, Windows user-profile paths, known private workspace fragments, and machine names returned no personal identifiers or local user paths.
3. A focused scan for named-user Windows path examples returned no matches.
4. `rg -n -F 'Public Release Checklist' README.md docs/RELEASE.md docs/PUBLIC_RELEASE.md` confirmed the new checklist is linked.
5. `./scripts/verify-api-doc-commands.sh` passed.
6. `npm run check` passed.
7. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
8. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Future local artifacts could be accidentally committed.
   - **Mitigation**: `docs/PUBLIC_RELEASE.md` documents the tracked-file metadata scan before public release.
2. **Risk**: README documentation links become hard to scan as docs grow.
   - **Mitigation**: Links are grouped by reader task instead of one flat list.

## Decisions

1. Keep ignored local `.DS_Store`, `dist/`, `node_modules/`, `.tmp/`, and `/pdf/` artifacts out of the PR because they are not tracked.
2. Treat the localhost URL in `src-tauri/tauri.conf.json` as an acceptable Tauri dev-server setting, not a personal identifier.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
