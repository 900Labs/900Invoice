# Sprint 051: Backup Format Docs Parity

- **Sprint ID**: `051`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The offline-first ADR still described backup as a prompted 30-day workflow that wrote a raw SQLite database file. The current Settings workflow exports a versioned JSON snapshot to a selected `.json` file and restore consumes that JSON payload additively. The Rust command comment also still described the backup as a base64-encoded binary blob even though the command returns JSON.

## Goals

1. Align backup strategy documentation with the current JSON backup and restore implementation.
2. Remove the unimplemented regular backup prompt claim.
3. Correct the stale Rust doc comment on `backup_database`.

## In Scope

1. `docs/adr/004-offline-first-architecture.md` backup strategy wording.
2. `src-tauri/src/commands/import_export.rs` command comment.
3. Changelog and sprint documentation.

## Out of Scope

1. Changing the backup file format.
2. Adding reminder prompts.
3. Adding binary SQLite database backup export.

## Deliverables

1. The ADR says Settings backup exports a versioned JSON data snapshot.
2. The ADR says restore consumes a selected JSON backup additively.
3. The ADR distinguishes the live SQLite database file from the Settings backup format.
4. The `backup_database` command comment matches its JSON return value.

## Validation

1. `rg -n -F -e 'Prompts users to create a backup every 30 days' -e 'standard SQLite database file' -e 'base64-encoded binary blob' docs/adr/004-offline-first-architecture.md src-tauri/src/commands/import_export.rs` returned no matches.
2. `./scripts/verify-api-doc-commands.sh` passed.
3. `npm run check` passed.
4. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
5. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Users may still expect raw SQLite backup export because live data is stored in SQLite.
   - **Mitigation**: The ADR now states that live data is SQLite while Settings backup is a JSON snapshot.
2. **Risk**: Backup reminder expectations remain ambiguous.
   - **Mitigation**: The unimplemented reminder claim was removed; reminder design can be handled as a separate feature.

## Decisions

1. Keep documenting the existing JSON backup format instead of adding a new binary SQLite backup feature.
2. Preserve the ADR's offline-first storage decision while correcting the backup workflow details that changed after implementation.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
