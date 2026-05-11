# Sprint 033: Exchange Rate Snapshots

- **Sprint ID**: `033`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README promised exchange-rate caching and invoice-level audit snapshots. The schema already had `exchange_rate_to_usd` and `exchange_rate_date`, but new invoices could leave those fields empty because default rates were not seeded on app startup and invoice commands did not fill snapshots.

## Goals

1. Seed offline default exchange rates during app startup.
2. Stamp invoices with a cached rate-to-USD snapshot.
3. Cover create, update, finalize, duplicate, and recurring generation flows.

## In Scope

1. Add a shared `exchange_rate_snapshot` service.
2. Use cached direct or inverse rates to build a USD snapshot.
3. Fill missing snapshots for invoice creation and finalization.
4. Refresh draft snapshots when invoice currency or issue date changes.
5. Stamp duplicated and recurring-generated invoices with fresh snapshots.
6. Update API, architecture, changelog, and sprint records.

## Out of Scope

1. Live network exchange-rate fetching.
2. Multi-currency accounting consolidation.
3. UI for editing invoice snapshot fields manually.

## Deliverables

1. Startup default-rate seeding.
2. Shared exchange-rate snapshot service with regression coverage.
3. Invoice command and recurring scheduler snapshot integration.
4. Documentation of the snapshot contract.

## Validation

1. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo check --manifest-path src-tauri/Cargo.toml` passed.
2. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml exchange_rate -- --nocapture` passed.
3. `npm run check` passed.
4. `npm run build` passed.
5. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml` passed.
6. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.
7. `./scripts/verify-api-doc-commands.sh` passed.
8. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
9. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: A historical invoice could be revalued if reports use the latest rate instead of the invoice rate.
   - **Mitigation**: invoice rows now persist `exchange_rate_to_usd` and `exchange_rate_date` when the invoice is created or generated.
2. **Risk**: Missing cached rates could block invoice creation.
   - **Mitigation**: missing rates leave the snapshot nullable; startup seeds defaults so supported currencies have offline fallback rates.

## Decisions

1. Store snapshots to USD because the schema already models `exchange_rate_to_usd`.
2. Use inverse cached rates when only `USD -> currency` exists so seeded default rows can support both conversion directions.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
