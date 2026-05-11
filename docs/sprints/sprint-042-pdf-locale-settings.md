# Sprint 042: PDF Locale Settings

- **Sprint ID**: `042`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README promises locale-aware date and number formatting, and Settings exposes paper size, locale, and date-format controls. The frontend preview used those settings, but the native PDF command path hardcoded `a4` and `en`, while the PDF renderer ignored locale for exported date and number formatting.

## Goals

1. Make native PDF export use persisted paper size, locale, and date-format settings.
2. Apply locale-aware decimal and thousands separators in PDF/HTML invoice rendering.
3. Keep the native PDF font limitation explicit in documentation.

## In Scope

1. Read and normalize `paper_size`, `locale`, and `date_format` in PDF commands.
2. Pass render settings into `generate_invoice_pdf_bytes` and `generate_invoice_html`.
3. Format PDF/HTML invoice issue dates, due dates, currency, quantities, and tax percentages with the selected locale/date settings.
4. Add regression coverage for settings parsing, Letter page output, and French date/number formatting.
5. Update README, PDF rendering docs, architecture notes, historical Typst template notes, I18N docs, and changelog.

## Out of Scope

1. Full translated static labels inside native PDF exports.
2. Full RTL shaping in native PDF output.
3. Reworking the frontend invoice preview modal layout.

## Deliverables

1. `generate_invoice_pdf` and `get_pdf_preview_data` no longer hardcode paper size or locale.
2. Native PDF media boxes follow saved paper-size settings.
3. PDF/HTML renderers apply saved locale/date formatting to invoice dates and numeric values.
4. Documentation describes the native PDF compatibility boundary.

## Validation

1. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml` passed with 80 tests.
2. `npm run check` passed.
3. `npm run build` passed.
4. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo check --manifest-path src-tauri/Cargo.toml` passed.
5. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.
6. `./scripts/verify-api-doc-commands.sh` passed.
7. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
8. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Unsupported setting values could break export.
   - **Mitigation**: PDF command settings are normalized to supported defaults.
2. **Risk**: Native PDF text rendering could produce unreadable non-ASCII glyphs.
   - **Mitigation**: Native PDF export continues using ISO currency codes and ASCII-compatible month labels while documenting full RTL PDF shaping as a future renderer enhancement.

## Decisions

1. Use locale-specific numeric separators in native PDF, but keep ISO currency codes for built-in PDF font compatibility.
2. Keep native PDF static labels in English for this sprint; translated PDF labels require a larger renderer/i18n contract.
3. Treat unsupported paper sizes, locales, and date formats as default settings rather than command errors.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
