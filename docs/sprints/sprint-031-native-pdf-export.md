# Sprint 031: Native PDF Export

- **Sprint ID**: `031`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README and architecture docs promised professional PDF generation, but `generate_invoice_pdf` returned base64-encoded HTML and the invoice preview only exposed `window.print()`. Users needed a real `.pdf` export path from the desktop app.

## Goals

1. Make `generate_invoice_pdf` return actual PDF bytes.
2. Add a native save flow from invoice preview.
3. Update the PDF documentation to match the implemented engine.

## In Scope

1. Generate a self-contained PDF document from invoice, client, tax, payment, and business data.
2. Keep the HTML renderer for live preview and print workflows.
3. Decode the base64 PDF response in the frontend and write it through the Tauri filesystem plugin.
4. Add PDF regression tests for real PDF output and literal-string escaping.
5. Correct README, API, architecture, rendering, and i18n docs that described the old Typst path.

## Out of Scope

1. Multi-template selection.
2. Embedded custom fonts or full RTL PDF text shaping.
3. External browser, Typst, or wkhtmltopdf based conversion.

## Deliverables

1. Native Rust PDF byte generation in `pdf_engine`.
2. Invoice preview Download button using native save dialogs.
3. PDF command contract updated from HTML payload to PDF bytes.
4. Documentation aligned with the shipped renderer.

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

1. **Risk**: Built-in PDF fonts do not support every international script.
   - **Mitigation**: native PDF export uses ISO currency codes and sanitized portable text; HTML preview remains available for richer WebView rendering. Full RTL shaping remains documented as future work.
2. **Risk**: Native PDF layout can diverge from the HTML preview.
   - **Mitigation**: both renderers live in `pdf_engine`, share formatting helpers, and have regression tests for command-level PDF output.

## Decisions

1. Keep PDF export dependency-free for this sprint instead of introducing a new compiler or browser conversion dependency.
2. Retain the HTML renderer as the preview and print surface while making the explicit Download action write real PDF bytes.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
