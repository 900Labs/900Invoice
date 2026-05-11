# Sprint 030: Recurring Generation Parity

- **Sprint ID**: `030`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Recurring invoice generation had two backend paths. The command exposed to the frontend created draft invoices directly, but it did not copy persisted invoice tax rows and could leave generated invoices with incomplete detail parity. The scheduler service already owned missed-job recovery and schedule advancement, so the command path needed to use that shared behavior.

## Goals

1. Make manual recurring generation use the same scheduler service as automated due processing.
2. Preserve template totals, line items, and invoice tax rows on generated invoices.
3. Keep frontend invoice state current after the user triggers generation.

## In Scope

1. Route `generate_due_recurring` through `recurring_scheduler::process_all_due`.
2. Return generated invoices with line items, taxes, and payments loaded.
3. Copy template invoice tax rows when generating recurring invoices.
4. Support the persisted `annually` frequency value and the existing `biweekly` model option.
5. Refresh recurring schedules and invoice store state after manual generation.

## Out of Scope

1. New recurring UI workflows or schedule editing screens.
2. Email delivery for `auto_send` schedules.
3. Replacing the app startup scheduler mechanism.

## Deliverables

1. Shared recurring generation path for frontend command and scheduler service.
2. Regression coverage for annual/biweekly date calculation and tax-row copying.
3. Updated API reference for the detailed generated invoice response.
4. Changelog and sprint handoff documentation.

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

1. **Risk**: Generated invoices can drift from finalized template totals if generation recalculates taxes differently from the saved template.
   - **Mitigation**: generation copies the template invoice totals and persisted tax summary rows as the finalized contract.
2. **Risk**: Frontend invoice lists can stay stale after manual generation.
   - **Mitigation**: the recurring view reloads both schedules and invoices after the command succeeds.

## Decisions

1. Keep generation in the scheduler service and make the command a thin adapter so future scheduler fixes apply to manual generation too.
2. Return `InvoiceWithDetails[]` from `generate_due_recurring` because the command now loads generated invoices through the same detailed invoice query used elsewhere.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
