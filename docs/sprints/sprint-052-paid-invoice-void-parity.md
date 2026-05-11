# Sprint 052: Paid Invoice Void Parity

- **Sprint ID**: `052`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README advertises the invoice lifecycle as Draft -> Finalized -> Sent -> Paid -> Void. The Rust `void_invoice` command already allows any non-void invoice to be voided, but the invoice detail UI hid the Void action for invoices in the Paid status. That left the user-facing app short of the documented lifecycle even though the backend command supported it.

## Goals

1. Make the invoice detail UI expose the Void action for paid invoices.
2. Keep the existing confirmation dialog and backend validation as the source of truth.
3. Improve void-action feedback so failed backend calls show an error instead of unconditional success.

## In Scope

1. `src/components/invoices/InvoiceDetailView.svelte` action visibility and handler feedback.
2. Changelog and sprint documentation.

## Out of Scope

1. Changing backend invoice transition rules.
2. Changing payment history or report treatment for void invoices.
3. Adding a separate credit-note workflow.

## Deliverables

1. Paid invoices now show the Void action in the invoice detail view.
2. Void action visibility now matches the backend rule: any non-void invoice can be voided.
3. The void handler reports errors when the backend rejects the action.

## Validation

1. `rg -n -F "invoice.status !== 'Void' && invoice.status !== 'Paid'" src/components/invoices/InvoiceDetailView.svelte` returned no matches.
2. `./scripts/verify-api-doc-commands.sh` passed.
3. `npm run check` passed.
4. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
5. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Voiding a paid invoice may need a richer accounting workflow later.
   - **Mitigation**: This sprint only exposes the backend-supported transition; future credit-note or refund handling can refine paid-void semantics.
2. **Risk**: Users could void an invoice unintentionally.
   - **Mitigation**: The existing confirmation dialog remains in place before the backend command runs.

## Decisions

1. Preserve the backend transition policy and align the UI with it instead of changing README lifecycle wording.
2. Treat the backend response as authoritative in the frontend handler.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
