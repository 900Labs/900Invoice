# Sprint 056: Invoice Numbering Transaction Parity

- **Sprint ID**: `056`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

ADR 003 still described an older `sequence_counters` design and pointed contributors to `src-tauri/src/services/numbering.rs`. The current schema uses `invoice_sequences` and the service lives in `src-tauri/src/services/invoice_numbering.rs`. The finalization command also advanced the sequence in a separate transaction before setting the invoice number and finalizing the invoice, which left a small failure window between counter advancement and invoice updates.

## Goals

1. Keep sequence advancement and invoice finalization in the same SQLite transaction.
2. Add a transaction-scoped numbering helper for callers that already own a write transaction.
3. Align ADR 003 with the current `invoice_sequences` schema and service path.

## In Scope

1. `finalize_invoice` transaction flow.
2. `src-tauri/src/services/invoice_numbering.rs` transaction helper and tests.
3. `docs/adr/003-gap-free-invoice-numbering.md` implementation details.
4. Changelog and sprint documentation.

## Out of Scope

1. Recurring scheduler numbering flow.
2. Replacing manual SQLite transaction statements with `rusqlite::Transaction`.
3. Changing invoice sequence settings UI.

## Deliverables

1. `finalize_invoice` now wraps invoice number assignment, total recalculation, exchange-rate snapshotting, and status update in one `BEGIN IMMEDIATE` transaction.
2. `generate_invoice_number_in_transaction` advances a sequence without committing a nested transaction.
3. A unit test verifies transaction-scoped generation rolls back with the caller.
4. ADR 003 documents `invoice_sequences`, current formatting options, and the live service path.

## Validation

1. `cargo fmt --manifest-path src-tauri/Cargo.toml` passed.
2. `rg -n -F -e 'sequence_counters' -e 'src-tauri/src/services/numbering.rs' docs/adr/003-gap-free-invoice-numbering.md` returned no matches.
3. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml invoice_numbering -- --nocapture` passed.
4. `./scripts/verify-api-doc-commands.sh` passed.
5. `npm run check` passed.
6. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
7. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Manual transaction handling can leave transactions open if error paths are missed.
   - **Mitigation**: `finalize_invoice` centralizes commit/rollback handling around one inner helper result.
2. **Risk**: Other numbering callers still use their own transaction scope.
   - **Mitigation**: The default generator retains its existing transaction wrapper; recurring scheduler transaction unification is explicitly out of scope.

## Decisions

1. Add a transaction-scoped helper rather than changing the default generator behavior used by existing callers.
2. Update ADR 003 to describe the current schema and finalization path instead of preserving the obsolete `sequence_counters` text.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
