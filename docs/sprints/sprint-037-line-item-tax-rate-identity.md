# Sprint 037: Line Item Tax-Rate Identity

- **Sprint ID**: `037`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README promises withholding tax support. The UI allowed selecting a specific tax rate, but line items only persisted `tax_rate_bps`. When two active rates shared a percentage, such as GST 5% and WHT 5%, recalculation could match the wrong tax definition and lose the withholding flag.

## Goals

1. Persist selected tax-rate identity on invoice line items.
2. Prefer `tax_rate_id` during tax recalculation while keeping `tax_rate_bps` fallback for older rows.
3. Seed built-in withholding tax rates with `is_withholding=true`.

## In Scope

1. Add `tax_rate_id` to the line-item schema and Rust/TypeScript contracts.
2. Preserve `tax_rate_id` through create, update, duplicate, recurring generation, backup, and restore flows.
3. Mark seeded Kenya WHT and Nigeria WHT rates as withholding.
4. Regression coverage for duplicate-percentage withholding calculation and migration seeding.

## Out of Scope

1. Product catalog tax-rate identity storage.
2. Country-specific tax-rate filtering in invoice editors.
3. Historical invoice tax row rewrites.

## Deliverables

1. Line-item `tax_rate_id` persistence.
2. Tax calculator identity-first rate matching.
3. Migration backfill for the new column and WHT seed flags.
4. API, changelog, and sprint documentation updates.

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

1. **Risk**: Existing databases lack the new line-item column.
   - **Mitigation**: migrations add `tax_rate_id` when missing and keep `tax_rate_bps` fallback for older rows.
2. **Risk**: Existing WHT seed rows may already exist with `is_withholding=false`.
   - **Mitigation**: migrations explicitly update known WHT seed IDs to `is_withholding=1`.

## Decisions

1. Keep `tax_rate_bps` on line items for historical compatibility and display.
2. Do not rewrite finalized invoice tax rows; the fix applies when draft line items are recalculated or new invoices are created.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
