# Sprint 048: Client CSV Round Trip

- **Sprint ID**: `048`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README advertises client/product CSV import/export. Client CSV export writes `tax_id`, `currency_code`, `payment_terms_days`, and `notes`, but client CSV import still read the older layout without `tax_id` or `notes`. Re-importing an exported client CSV could therefore store the tax ID in `currency_code`, drop notes, and lose payment terms.

## Goals

1. Make client CSV import round-trip the current exported header.
2. Preserve client tax IDs and notes during CSV import.
3. Keep older client CSV files without `tax_id` and `notes` importable.

## In Scope

1. Client CSV import parsing.
2. Client CSV export/import helper extraction for regression tests.
3. API docs, changelog, and sprint documentation.

## Out of Scope

1. Changing client CSV export column order.
2. Adding UI-level CSV column mapping.
3. Changing product CSV behavior.

## Deliverables

1. `import_clients_csv` reads fields by header name and falls back to the legacy column positions.
2. Current client CSV exports round-trip tax ID, currency, payment terms, and notes.
3. Legacy client CSV imports still preserve currency and payment terms.

## Validation

1. `npm run check` passed.
2. `npm run build` passed.
3. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo check --manifest-path src-tauri/Cargo.toml` passed.
4. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml` passed.
5. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.
6. `./scripts/verify-api-doc-commands.sh` passed.
7. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
8. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Existing legacy client CSV files may not include new columns.
   - **Mitigation**: Import remains header-aware with legacy fallback positions for the old `currency_code` and `payment_terms_days` columns.
2. **Risk**: Invalid payment terms may silently become defaults.
   - **Mitigation**: Import now reports row-level errors for non-numeric or negative `payment_terms_days` values.

## Decisions

1. Keep the current exported client CSV header stable.
2. Use the same header-aware parsing approach as product CSV import.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
