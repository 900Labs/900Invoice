# Sprint 038: Duplicate Invoice Totals

- **Sprint ID**: `038`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README promises invoice duplication. The command copied line items into a new draft invoice, but it returned the newly inserted invoice before recalculating totals and persisted tax rows. Because inserted invoices start with zero totals, duplicated invoices could appear with line items but stale subtotal, tax, and total amounts.

## Goals

1. Recalculate duplicated invoice totals after copying line items.
2. Persist duplicated invoice tax rows through the same calculator path used by normal line-item edits.
3. Add regression coverage for duplicated invoices with tax-bearing line items.

## In Scope

1. Refactor duplicate invoice command logic into a testable internal helper.
2. Run `recalculate_invoice_totals` before returning duplicated invoice details.
3. Assert duplicated draft subtotal, tax amount, total, copied tax-rate identity, and persisted tax rows.

## Out of Scope

1. Changing invoice numbering behavior for duplicated drafts.
2. Changing invoice-level discount calculation semantics.
3. Editing product catalog tax defaults.

## Deliverables

1. Duplicate invoice command returns recalculated draft totals.
2. Duplicate invoice command persists recalculated invoice tax rows.
3. Regression test coverage in the Rust command module.
4. Changelog and sprint documentation updates.

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

1. **Risk**: Duplication could diverge from normal invoice recalculation behavior.
   - **Mitigation**: The command now calls the shared `recalculate_invoice_totals` helper after copying line items.
2. **Risk**: Tax rows with duplicate percentages could be regenerated with the wrong tax definition.
   - **Mitigation**: The regression asserts copied `tax_rate_id` identity and the recalculated `invoice_taxes` row.

## Decisions

1. Keep duplicated invoices as draft invoices with no invoice number, matching existing behavior.
2. Use the existing line-item insert path so copied line totals are recomputed consistently.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
