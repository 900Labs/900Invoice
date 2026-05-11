# Sprint 027: Frontend IPC Contract Parity

- **Sprint ID**: `027`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Runtime audit of README-backed workflows found that several Svelte stores were calling Tauri commands with frontend-shaped payloads (`data`, camelCase model fields, and PascalCase statuses) while the Rust command layer expects specific command arguments and serde model fields. Static builds could pass while core product workflows failed at runtime.

## Goals

1. Restore IPC contract parity for primary app workflows.
2. Preserve the existing frontend camelCase domain model while mapping backend snake_case JSON at the store boundary.
3. Complete the advertised invoice lifecycle by exposing a sent-state command and UI action.
4. Keep frontend dependency installs reproducible for Svelte validation.

## In Scope

1. Map clients, products, taxes, settings, exchange rates, recurring schedules, invoices, line items, and payments between frontend and backend contracts.
2. Create/update invoice line items through the line-item commands after invoice creation/update.
3. Refresh invoice details after lifecycle and payment mutations so status, payments, and balances stay current.
4. Add `mark_invoice_sent` and expose it from the invoice detail view.
5. Persist invoice tax rows during invoice total recalculation.
6. Ensure model-backed invoice tax calculation applies each line item's stored tax rate instead of every active tax rate.
7. Pin `esrap@2.2.2` to avoid the broken `2.2.3` package payload.

## Out of Scope

1. Native file-picker import/export UX.
2. Report-level redesigns or analytics model changes.
3. Database schema migrations.
4. Country-specific tax-rule redesigns beyond the line-item tax-rate parity fix.

## Deliverables

1. Store-level frontend/backend payload mappers.
2. Corrected invoke argument names for affected workflows.
3. Backend `mark_invoice_sent` command registration and API documentation.
4. Updated tax summary regression coverage.
5. Updated frontend dependency override for `esrap`.
6. Changelog and sprint documentation updates.

## Validation

1. `npm run check` passed.
2. `npm run build` passed.
3. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo check --manifest-path src-tauri/Cargo.toml` passed.
4. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml` passed with 60 tests.
5. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.
6. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.

## Risks and Mitigations

1. **Risk**: Tauri command argument names and nested serde field names use different casing rules.
   - **Mitigation**: documented the boundary rule in `docs/API.md` and kept explicit mappers in the store layer.
2. **Risk**: Product tax associations are stored by basis points, while the UI uses tax rate IDs.
   - **Mitigation**: stores preserve the backend basis-point value and resolve IDs from active tax rates for form controls.
3. **Risk**: `esrap@2.2.3` can install successfully while missing exported files required by Svelte.
   - **Mitigation**: pinned `esrap@2.2.2`, which contains `esrap/languages/ts`.

## Decisions

1. Keep frontend PascalCase invoice statuses to minimize component churn and map backend status strings in `invoiceStore`.
2. Use store-level adapters instead of broad backend serde renames because the Rust models are already used by persistence, PDF, and API docs.
3. Fetch invoice details after invoice mutations because list responses do not include related client, line-item, tax, or payment data.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
