# Sprint 034: Product CSV Portability

- **Sprint ID**: `034`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README promises CSV import/export under Client & Product Management. Client CSV import/export already existed, but the product/service catalog could only be backed up through the full JSON database export.

## Goals

1. Add product CSV import/export commands to the Tauri command surface.
2. Expose product CSV import/export in Settings.
3. Preserve inactive products and sanitize spreadsheet-evaluable CSV cells on export.

## In Scope

1. Product CSV import with `name,description,default_price,default_currency,default_tax_rate_bps,unit,is_active`.
2. Product CSV export for all products, including inactive rows.
3. Price conversion between CSV major units and stored minor units.
4. API, README, architecture, changelog, and sprint documentation updates.

## Out of Scope

1. Bulk product update or duplicate detection.
2. XLSX import/export.
3. Product catalog schema changes.

## Deliverables

1. `import_products_csv` and `export_products_csv` backend commands.
2. Settings import/export controls for product CSV files.
3. Regression tests for import behavior and CSV export sanitization.

## Validation

1. `cargo fmt --manifest-path src-tauri/Cargo.toml` passed.
2. `npm run check` passed.
3. `npm run build` passed.
4. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo check --manifest-path src-tauri/Cargo.toml` passed.
5. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml` passed.
6. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.
7. `./scripts/verify-api-doc-commands.sh` passed.
8. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
9. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Spreadsheet apps may execute exported product names or descriptions as formulas.
   - **Mitigation**: Product CSV export uses the shared CSV sanitizer that prefixes formula-like cells.
2. **Risk**: Minor-unit price storage can be confusing in CSV files.
   - **Mitigation**: CSV uses human-readable major-unit prices and converts by currency decimals on import/export.

## Decisions

1. Include inactive products in export so the catalog can round-trip completely.
2. Keep import additive and append-only, matching existing client CSV import behavior.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
