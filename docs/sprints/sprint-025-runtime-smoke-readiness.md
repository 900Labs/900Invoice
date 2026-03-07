# Sprint 025: Runtime Smoke Readiness

- **Sprint ID**: `025`
- **Date**: 2026-03-07
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Quality and governance automation were already hardened, but maintainers still needed a single, repeatable runtime smoke command that can run in low-resource environments before public release.

## Goals

1. Add a single runtime smoke script for practical release readiness.
2. Document smoke profiles and environment controls in a dedicated runbook.
3. Ensure low-resource/legacy hardware validation stays first-class.

## In Scope

1. Add `scripts/verify-runtime-smoke.sh`.
2. Add `docs/RUNTIME_SMOKE.md` with profile usage and failure handling.
3. Update contributor/quality docs and README index to include runtime smoke guidance.
4. Execute smoke checks and record validation evidence.
5. Stabilize frontend smoke dependency resolution and Tauri metadata prerequisites uncovered during smoke execution.

## Out of Scope

1. New product features.
2. Database schema or API surface changes.
3. Governance workflow/policy contract changes.

## Deliverables

1. Runtime smoke script with:
   - `baseline` and `full` profiles
   - optional dependency install (`INSTALL_NODE_DEPS=1`)
   - legacy hardware mode (`LEGACY_HARDWARE=1`)
   - dedicated cargo target dir and tmp dir (`CARGO_TARGET_DIR`, `SMOKE_TMPDIR`) for low lock-contention and safer temporary-file handling
2. Runtime smoke runbook with:
   - recommended commands for pre-PR, legacy hardware, and release-grade checks
   - failure-handling guidance
   - PR evidence examples
3. Frontend/runtime stability hardening:
   - pinned `@sveltejs/vite-plugin-svelte` to `6.0.0`
   - pinned `@sveltejs/vite-plugin-svelte-inspector` to `5.0.0`
   - scoped `npm run check` to `--workspace src` for deterministic diagnostics
   - repaired `src-tauri/icons/icon.png` to a valid RGBA PNG required by `tauri::generate_context!()`
4. Documentation updates:
   - `README.md`
   - `docs/QUALITY_GATE.md`
   - `CONTRIBUTING.md`
   - `CHANGELOG.md`

## Validation

1. `bash -n scripts/verify-runtime-smoke.sh` passed.
2. `npm run check` passed.
3. `INSTALL_NODE_DEPS=1 SMOKE_PROFILE=baseline LEGACY_HARDWARE=1 ./scripts/verify-runtime-smoke.sh` passed.
4. `INSTALL_NODE_DEPS=0 SMOKE_PROFILE=full ./scripts/verify-runtime-smoke.sh` passed.

## Risks and Mitigations

1. **Risk**: local Node dependency state can cause false-negative smoke failures.
   - **Mitigation**: script supports `INSTALL_NODE_DEPS=1` and fails fast when `node_modules` is missing.
2. **Risk**: low-memory machines may fail concurrent Rust builds.
   - **Mitigation**: legacy mode enforces single-job cargo and reduced npm overhead.

## Decisions

1. Keep runtime smoke as a dedicated script instead of adding more one-off commands to `docs/QUALITY_GATE.md`.
2. Keep clippy only in `full` profile so maintainers can run a faster baseline smoke loop during iteration.
3. Preserve explicit legacy-hardware mode to align with 900Invoice accessibility goals in developing economies.

## Follow-Ups

1. Optionally add a CI workflow job that runs baseline runtime smoke on release candidates.
2. Optionally add Windows/macOS-specific smoke command snippets to the runbook.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
