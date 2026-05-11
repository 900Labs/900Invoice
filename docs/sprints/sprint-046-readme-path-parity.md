# Sprint 046: README Path Parity

- **Sprint ID**: `046`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README source-build command now correctly uses `cd 900Invoice`, but the project-structure block still showed the root folder as `900invoice/`. The quick contribution ideas also pointed tax-rate contributors to `src-tauri/src/services/tax.rs`, a file that does not exist in the current codebase.

## Goals

1. Keep README path references aligned with the live repository.
2. Remove a dead tax-rate contribution path from the contributor quick-start section.
3. Keep the documentation-only change small and auditable.

## In Scope

1. README project-structure root casing.
2. README tax-rate contribution path.
3. Changelog and sprint documentation.

## Out of Scope

1. Runtime tax-rate changes.
2. Restructuring tax seed management.
3. Broader README copy edits.

## Deliverables

1. README project tree starts with `900Invoice/`.
2. README quick contribution tax-rate path points to `src-tauri/src/db/migrations.rs`.

## Validation

1. `rg -n "900invoice/|src-tauri/src/services/tax.rs" README.md` returned no matches.
2. `./scripts/verify-api-doc-commands.sh` passed.
3. `npm run check` passed.
4. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
5. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Contributors may still need more context than a path gives them.
   - **Mitigation**: This sprint fixes the dead path only; broader contribution guidance can be handled separately if needed.

## Decisions

1. Point tax-rate seed contributors to the migration seed file because that is where default tax rates are currently inserted.
2. Keep the README source-build and project-structure casing consistent with the actual clone directory.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
