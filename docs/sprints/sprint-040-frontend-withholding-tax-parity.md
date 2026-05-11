# Sprint 040: Frontend Withholding Tax Parity

- **Sprint ID**: `040`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README promises withholding tax support. The backend calculator persisted withholding rows correctly and deducted them from invoice totals, but the frontend tax summary contract did not carry `is_withholding`. As a result, invoice detail, preview, and editor summaries displayed WHT as a positive tax line, and the editor draft total added WHT before save.

## Goals

1. Carry withholding identity through frontend `TaxLine` data.
2. Display withholding tax rows as deductions in shared invoice summaries.
3. Match editor draft totals to backend withholding totals in exclusive and inclusive tax modes.

## In Scope

1. Add `isWithholding` to frontend tax-line mapping.
2. Prefer `tax_rate_id` when deriving tax report base amounts from line items.
3. Update editor tax totals to include only non-withholding taxes and subtract withholding.
4. Update `TaxSummary` rendering for withholding rows.

## Out of Scope

1. Changing backend tax calculation semantics.
2. Rewriting historical invoice tax rows.
3. Adding tax-specific frontend unit-test infrastructure.

## Deliverables

1. Frontend tax-line contract includes withholding identity.
2. Shared invoice summaries render WHT as negative/deducted rows.
3. Editor totals match backend WHT behavior before save/finalize.
4. Changelog and sprint documentation updates.

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

1. **Risk**: Frontend tax totals could diverge from backend persisted totals.
   - **Mitigation**: The editor now follows the backend rule of summing non-withholding tax into `taxTotalMinor` and subtracting withholding from total.
2. **Risk**: Duplicate tax percentages could still distort tax report bases.
   - **Mitigation**: Tax-line base mapping now prefers `tax_rate_id` before falling back to basis points.

## Decisions

1. Keep `taxTotalMinor` as non-withholding tax, matching backend `invoice.tax_amount_minor`.
2. Keep saved invoice data authoritative; frontend changes only affect display and draft preview parity.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
