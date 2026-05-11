# Sprint 053: Tauri Docs Claim Parity

- **Sprint ID**: `053`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README used precise Tauri binary-size and idle-memory numbers as if they were measured 900Invoice outputs, but the current release workflow does not publish platform bundles and the repo does not keep measured runtime footprint evidence. ADR 001 also listed auto-update as part of the current Tauri app capability even though deployment docs correctly mark updater delivery as future hardening, and it pointed contributors to command registration in `src-tauri/src/main.rs` even though commands are registered in `src-tauri/src/lib.rs`.

## Goals

1. Keep README Tauri benefits accurate without unmeasured binary/RAM promises.
2. Align ADR 001 updater wording with the current no-updater implementation.
3. Correct ADR 001 command registration path.

## In Scope

1. README Tauri tech stack and comparison wording.
2. `docs/adr/001-tauri-v2-desktop-framework.md` framework capability and contributor notes.
3. Changelog and sprint documentation.

## Out of Scope

1. Measuring packaged binary size or idle memory.
2. Adding updater dependencies or updater configuration.
3. Changing the Tauri command registry.

## Deliverables

1. README describes Tauri as a small native shell with lower idle-memory and faster-startup profile instead of fixed app measurements.
2. ADR 001 describes updater support as future plugin/configuration work, not a currently enabled app capability.
3. ADR 001 points command registration readers to `src-tauri/src/lib.rs`.

## Validation

1. `rg -n -F -e '2.5 MB binary' -e '30 MB RAM' -e 'Binary Size | 2.5' -e 'RAM (idle) | 30' -e 'src-tauri/src/main.rs' -e 'system dialogs, auto-update' README.md docs/adr/001-tauri-v2-desktop-framework.md` returned no matches.
2. `./scripts/verify-api-doc-commands.sh` passed.
3. `npm run check` passed.
4. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
5. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Removing precise numbers weakens the marketing comparison.
   - **Mitigation**: The README and ADR still explain the architectural reason Tauri is smaller and lighter: it uses the system WebView instead of bundling Chromium.
2. **Risk**: Future release builds may add measured footprint data.
   - **Mitigation**: Exact metrics can be restored once measured per-platform artifacts are published.

## Decisions

1. Do not publish exact size or idle-memory numbers until the project has reproducible package artifacts and measurement evidence.
2. Treat updater delivery as future release hardening until the repository includes the updater plugin and `tauri.conf.json` updater configuration.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
