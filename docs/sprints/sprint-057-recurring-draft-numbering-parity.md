# Sprint 057: Recurring Draft Numbering Parity

- **Sprint ID**: `057`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Recurring generation created new invoices as drafts but assigned an invoice number immediately by calling the default sequence generator. That conflicted with the gap-free numbering rule that draft invoices should not consume numbers until finalization. It also left recurring draft generation outside the finalization transaction hardening from Sprint 056.

## Goals

1. Keep recurring-generated drafts numberless until the user finalizes them.
2. Ensure recurring generation does not advance the invoice sequence.
3. Align architecture notes with the current invoice sequence table and void transition behavior.

## In Scope

1. `src-tauri/src/services/recurring_scheduler.rs` draft invoice insertion and tests.
2. `docs/ARCHITECTURE.md` lifecycle and sequence table references.
3. Changelog and sprint documentation.

## Out of Scope

1. Auto-send recurring invoice behavior.
2. Finalizing recurring-generated invoices automatically.
3. Changing recurring schedule advancement rules.

## Deliverables

1. Generated recurring invoices insert `NULL` for `invoice_number`.
2. Regression coverage verifies generated recurring invoices remain draft, have no invoice number, and do not advance the sequence preview.
3. Architecture docs refer to `invoice_sequences` and current void transition behavior.

## Validation

1. `cargo fmt --manifest-path src-tauri/Cargo.toml` passed.
2. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml recurring_scheduler -- --nocapture` passed.
3. `rg -n -F -e 'generate_invoice_number(conn, "default")' -e 'INV-{}' src-tauri/src/services/recurring_scheduler.rs` returned no matches.
4. `rg -n -F 'sequence_counters' docs/ARCHITECTURE.md docs/adr/003-gap-free-invoice-numbering.md` returned no matches.
5. `./scripts/verify-api-doc-commands.sh` passed.
6. `npm run check` passed.
7. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
8. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Users may expect generated recurring drafts to already display an invoice number.
   - **Mitigation**: This matches the app-wide draft behavior; numbers are assigned when drafts are finalized.
2. **Risk**: Auto-send semantics remain unused.
   - **Mitigation**: Auto-send behavior is explicitly out of scope; this sprint preserves current draft generation semantics.

## Decisions

1. Do not allocate invoice numbers for generated drafts.
2. Keep recurring generation as draft creation only; finalization remains the single place where numbering occurs.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
