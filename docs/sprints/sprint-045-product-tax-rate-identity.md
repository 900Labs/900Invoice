# Sprint 045: Product Tax-Rate Identity

- **Sprint ID**: `045`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README promises a product/service catalog for quick invoicing. Product defaults stored only `default_tax_rate_bps`, while invoice line items now preserve `tax_rate_id` so withholding and same-percentage rates calculate correctly. A product configured with a same-percentage tax could therefore remap to the first active rate with that basis-point value when selected in an invoice.

## Goals

1. Preserve the selected product default tax-rate identity end to end.
2. Keep existing products and legacy product CSV imports compatible.
3. Ensure quick-invoiced line items receive the intended tax-rate ID.

## In Scope

1. Product database schema and migration.
2. Product Rust models, query mapping, JSON backup/restore, and CSV import/export.
3. Frontend product store snake_case/camelCase mapping.
4. API docs, changelog, and regression coverage.

## Out of Scope

1. Changing tax-rate seed data.
2. Adding tax-rate foreign-key constraints to historical product rows.
3. Reworking the product form UI.

## Deliverables

1. `products.default_tax_rate_id` is created for new databases and added to existing databases.
2. Product create/update/list/search preserve `default_tax_rate_id` while retaining `default_tax_rate_bps`.
3. Product CSV export includes `default_tax_rate_id`; import accepts both current and legacy headers.
4. JSON backup/restore preserves product default tax-rate identity.

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

1. **Risk**: Older product CSV files may not include the new tax ID column.
   - **Mitigation**: Import still falls back to the previous column order and basis-point field when `default_tax_rate_id` is absent.
2. **Risk**: Products may reference custom tax IDs that are not active in a restored or imported database.
   - **Mitigation**: The legacy basis-point value remains stored as a fallback for display and recalculation compatibility.

## Decisions

1. Store tax identity as an optional text column without adding a hard foreign key, matching the existing line-item tax identity approach.
2. Keep `default_tax_rate_bps` in the product contract for backward compatibility and fallback behavior.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
