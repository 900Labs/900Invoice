# Sprint 035: Revenue Client Breakdown

- **Sprint ID**: `035`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README promises revenue reports by period, client, and currency. The Reports view already had a period selector and currency totals, but it did not show or export a client-level revenue breakdown.

## Goals

1. Add client-and-currency revenue breakdowns to the revenue report.
2. Align revenue CSV export with the visible revenue aggregation.
3. Preserve currency-specific decimal handling in report CSV exports.

## In Scope

1. Revenue summary rows grouped by client and currency for paid invoices in the selected period.
2. Revenue CSV export with client, currency, invoice count, total revenue, and average invoice columns.
3. Currency-aware money formatting for revenue, tax, and aging CSV exports.
4. Translation and changelog updates.

## Out of Scope

1. New backend reporting commands.
2. Charting library replacement.
3. Cash-basis versus accrual-basis report configuration.

## Deliverables

1. Revenue report client breakdown table.
2. Revenue CSV export based on client-and-currency aggregation.
3. Report export formula-cell hardening for string fields.

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

1. **Risk**: Multi-currency totals could be mixed into one misleading number.
   - **Mitigation**: Revenue rows are grouped by both client and currency, and currency totals remain separate.
2. **Risk**: CSV exports could use incorrect decimals for zero-decimal currencies.
   - **Mitigation**: Report CSV money values now use the currency config decimal count.

## Decisions

1. Treat revenue as paid invoices for consistency with the existing revenue chart.
2. Keep the report client breakdown in the frontend because all required invoice data is already loaded there.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
