# Sprint 028: Data Portability Hardening

- **Sprint ID**: `028`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

README-backed data portability workflows were only partially functional. Settings import/export buttons invoked backend commands without selecting or writing files, and JSON restore only restored clients and products even though backup emitted a broader database snapshot.

## Goals

1. Make Settings import/export actions perform real local file IO through Tauri dialogs.
2. Make JSON backup and restore cover the same core offline data set.
3. Preserve additive restore behavior so existing rows are not dropped or overwritten.
4. Add regression coverage for restore completeness and duplicate safety.

## In Scope

1. Client CSV import from a selected file.
2. Client and invoice CSV export to selected save paths.
3. JSON backup export to a selected save path.
4. JSON restore from a selected backup file.
5. Backend backup coverage for invoice line items, invoice taxes, exchange rates, and invoice sequences.
6. Backend additive restore coverage for business profile, settings, tax rates, clients, products, invoices, line items, invoice taxes, payments, recurring schedules, exchange rates, and invoice sequences.

## Out of Scope

1. Destructive restore or full database replacement.
2. Binary SQLite backup file format changes.
3. Importing invoice CSV files.
4. Redesigning the Settings layout.

## Deliverables

1. Native file open/save wiring in `SettingsView`.
2. Complete JSON backup/restore table coverage for the current schema.
3. Per-table inserted row counts that reflect actual `INSERT OR IGNORE` effects.
4. Restore regression test covering invoice-related rows and duplicate restore safety.
5. Updated API and changelog documentation.

## Validation

1. `npm run check` passed.
2. `npm run build` passed.
3. `cargo fmt --manifest-path src-tauri/Cargo.toml` applied formatting.
4. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo check --manifest-path src-tauri/Cargo.toml` passed.
5. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml` passed with 61 tests.
6. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.
7. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.

## Risks and Mitigations

1. **Risk**: Additive restore can be mistaken for full replacement.
   - **Mitigation**: API docs explicitly state that restore uses `INSERT OR IGNORE` and does not overwrite existing rows.
2. **Risk**: Backup coverage drifts as tables are added.
   - **Mitigation**: restore regression now exercises invoice child tables, payments, recurring schedules, exchange rates, and invoice sequences.
3. **Risk**: UI can show stale data after import or restore.
   - **Mitigation**: Settings reloads affected stores after client import and full restore.

## Decisions

1. Keep the current versioned JSON backup format for this sprint to minimize migration risk.
2. Keep restore additive because this is the documented behavior and avoids destructive data loss.
3. Use Tauri dialog and filesystem plugins already present in the project instead of adding new dependencies.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
