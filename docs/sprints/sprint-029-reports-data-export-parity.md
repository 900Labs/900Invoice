# Sprint 029: Reports Data Export Parity

- **Sprint ID**: `029`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Reports were rendering from frontend invoice state, but loaded tax lines had `baseAmountMinor` set to zero. That made the tax filing report show collected tax without a taxable base. The reports CSV action also exported the same invoice rows regardless of the active report tab and used a browser download flow instead of the desktop file-save path used elsewhere.

## Goals

1. Populate tax report base amounts from persisted invoice details.
2. Make report CSV export reflect the active report tab.
3. Use native save dialogs for report exports in the desktop app.

## In Scope

1. Derive tax-line base amounts from invoice line items in `invoiceStore`.
2. Export revenue report rows with invoice totals and balances.
3. Export tax report rows with tax name, rate, currency, taxable base, and tax collected.
4. Export aging report rows with due date, bucket, currency, and balance due.
5. Route report CSV saves through Tauri dialog/filesystem plugins.

## Out of Scope

1. New chart types or visual report redesign.
2. Server-side report generation.
3. Multi-currency consolidation into a single reporting currency.

## Deliverables

1. Correct non-zero taxable base values in tax summary reports.
2. Active-tab report CSV export.
3. Native desktop save flow for report CSV files.
4. Changelog and sprint handoff documentation.

## Validation

1. `npm run check` passed.
2. `npm run build` passed.
3. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed with 61 Rust tests.

## Risks and Mitigations

1. **Risk**: Multiple tax rates can share the same basis-point value.
   - **Mitigation**: current product and line-item persistence stores tax by basis points, so the report mapper follows that persisted contract. A future schema can add line-level tax IDs if distinct same-rate taxes must be separated.
2. **Risk**: Browser-style downloads are unreliable in packaged desktop contexts.
   - **Mitigation**: report export now uses the same native save APIs as Settings export.

## Decisions

1. Keep report calculations client-side for this sprint because invoice details are already loaded into the frontend store.
2. Export active-tab data instead of a generic invoice list so CSV output matches the screen the user is viewing.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
