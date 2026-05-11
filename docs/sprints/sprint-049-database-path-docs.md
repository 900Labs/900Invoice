# Sprint 049: Database Path Docs

- **Sprint ID**: `049`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The deployment guide says the database path is determined by Tauri `app_data_dir()`, but its Linux and Windows examples used `900invoice` folders instead of the configured bundle identifier `com.900labs.invoice`. The same section also told users to find the path in Settings -> About -> Database Location, but the app does not expose that screen or locator.

## Goals

1. Align deployment database paths with Tauri app data directory semantics.
2. Remove the non-existent in-app database locator reference.
3. Keep the README's generic `{APP_DATA_DIR}/900invoice.db` guidance unchanged.

## In Scope

1. Linux and Windows database path examples in `docs/DEPLOYMENT.md`.
2. Database Location Reference wording in `docs/DEPLOYMENT.md`.
3. Changelog and sprint documentation.

## Out of Scope

1. Adding an in-app database location UI.
2. Changing the Tauri identifier.
3. Moving existing user databases.

## Deliverables

1. Linux database path uses `~/.local/share/com.900labs.invoice/900invoice.db`.
2. Windows database path uses `%APPDATA%\com.900labs.invoice\900invoice.db`.
3. The guide explains that the bundle identifier comes from `src-tauri/tauri.conf.json`.

## Validation

1. `rg -n -F -e 'Settings -> About' -e 'Settings → About' -e '%APPDATA%\900invoice' -e 'local/share/900invoice' docs/DEPLOYMENT.md` returned no matches.
2. `./scripts/verify-api-doc-commands.sh` passed.
3. `npm run check` passed.
4. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
5. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Platform path behavior may vary if Tauri changes path semantics.
   - **Mitigation**: The guide ties examples to the current configured bundle identifier and notes that Tauri `app_data_dir()` determines the location.
2. **Risk**: Users may still need a UI path locator.
   - **Mitigation**: This sprint corrects documentation only; a future UI locator can be added as a product feature.

## Decisions

1. Use `com.900labs.invoice` in platform examples because it is the live `identifier` in `src-tauri/tauri.conf.json`.
2. Do not claim an in-app database locator until one exists.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
