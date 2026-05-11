# Sprint 050: Recovery Docs Parity

- **Sprint ID**: `050`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The deployment guide's post-update recovery section claimed users could hold `Shift` during startup to skip database loading and then open Settings -> Restore Database. The Tauri setup path initializes the SQLite database before managing app state and does not implement a startup modifier bypass, so the recovery steps overstated runtime behavior.

## Goals

1. Remove the non-existent startup `Shift` database bypass from deployment recovery guidance.
2. Keep documented recovery paths aligned with the app's actual Settings restore control and local SQLite database behavior.
3. Preserve a safe fallback that tells users to move the database aside instead of relying on an unavailable in-app screen when startup is blocked.

## In Scope

1. `docs/DEPLOYMENT.md` post-update startup recovery wording.
2. Changelog and sprint documentation.

## Out of Scope

1. Adding a startup recovery mode.
2. Adding an in-app database file locator.
3. Changing database initialization or backup/restore implementation.

## Deliverables

1. The deployment guide no longer claims `Shift` skips database loading.
2. The guide distinguishes Settings-based restore from startup-blocked database-file recovery.
3. Users are told to move the database file out of the app data directory when startup is blocked.

## Validation

1. `rg -n -F -e 'Hold `Shift`' -e 'skip database loading' docs/DEPLOYMENT.md` returned no matches.
2. `./scripts/verify-api-doc-commands.sh` passed.
3. `npm run check` passed.
4. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
5. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Users may still want an easier startup recovery mode.
   - **Mitigation**: This sprint corrects documentation only; a real startup recovery feature can be designed separately.
2. **Risk**: Users may confuse moving the database aside with restoring a backup.
   - **Mitigation**: The guide states that moved data remains unavailable unless recovered or restored from backup.

## Decisions

1. Document the existing Settings restore path only for cases where the app can launch.
2. Prefer "move the database file" over "delete the database" so users do not discard potentially recoverable data.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
