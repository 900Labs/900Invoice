# Sprint 039: Currency-Safe Dashboard and Reports

- **Sprint ID**: `039`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README promises multi-currency support, revenue reports by period/client/currency, aging reports, and dashboard metrics. Revenue summaries already separated client and currency totals, but dashboard revenue/outstanding totals, aging bucket totals, and revenue period bars summed minor units from different currencies and displayed the result as the default currency.

## Goals

1. Keep dashboard revenue and outstanding totals grouped by invoice currency.
2. Keep aging bucket totals grouped by invoice currency.
3. Keep revenue period bars grouped by period and currency.

## In Scope

1. Shared frontend helper for currency-total grouping and formatted detail rows.
2. Multi-row dashboard stat cards for currency-specific totals.
3. Currency-specific aging bucket summaries.
4. Currency-specific revenue chart period bars.

## Out of Scope

1. Converting all totals into a reporting currency with exchange-rate snapshots.
2. Changing CSV export contracts.
3. Adding a charting library.

## Deliverables

1. `src/utils/currencyTotals.ts` helper for grouped totals.
2. Dashboard stat cards that display one row per currency.
3. Aging report bucket cards that display one row per currency.
4. Revenue period bars that remain separated by currency.
5. Changelog and sprint documentation updates.

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

1. **Risk**: Dashboard cards could become too dense when many currencies are active.
   - **Mitigation**: Stat cards render compact rows with wrapping amount text and fixed currency labels.
2. **Risk**: Revenue period chart could imply currency conversion.
   - **Mitigation**: Each bar is now keyed by both period and currency, and tooltips format with that bar's currency.

## Decisions

1. Prefer grouped currency display over implicit conversion because invoice exchange-rate snapshots are audit data, not a global reporting-currency policy.
2. Keep the zero state in the user's default currency so empty dashboards remain simple.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
