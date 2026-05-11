# Sprint 041: I18N Surface Cleanup

- **Sprint ID**: `041`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README promises six languages, translation-key coverage, RTL support, and locale-aware formatting. The locale bundles were complete, but several core UI surfaces still had hardcoded English strings, including shared accessibility labels, Settings import/export descriptions, line-item tooltips, and invoice preview payment labels.

## Goals

1. Move remaining core hardcoded English UI strings to translation keys.
2. Keep all six locale files in key-count parity.
3. Preserve existing behavior and layout.

## In Scope

1. Shared accessibility labels for search, modal close, spinner status, and toast controls.
2. Settings currency hint and import/export helper descriptions.
3. Invoice preview labels for invoice title, tax ID, payment information, bank, and account.
4. Line-item reorder/remove tooltips.
5. Locale entries for English, French, Spanish, Arabic, Swahili, and Hindi.

## Out of Scope

1. Reworking locale selection UX.
2. Translating user-entered data, product names, client names, or payment method values already stored in records.
3. Adding automated locale schema tooling.

## Deliverables

1. Core UI and accessibility text reads from `t(...)`.
2. All locale files contain 313 translation keys.
3. Changelog and sprint documentation updates.

## Validation

1. `node -e "...JSON.parse..."` for all locale files passed.
2. Locale key counts matched at 313 keys for all six files.
3. Hardcoded-string audit for the targeted strings returned no matches.
4. `npm run check` passed.
5. `npm run build` passed.
6. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo check --manifest-path src-tauri/Cargo.toml` passed.
7. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo test --manifest-path src-tauri/Cargo.toml` passed.
8. `CARGO_TARGET_DIR=/tmp/900invoice-target-check cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` passed.
9. `./scripts/verify-api-doc-commands.sh` passed.
10. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
11. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Adding keys to multiple locale files could introduce JSON drift.
   - **Mitigation**: Parsed every locale file and checked equal key counts after edits.
2. **Risk**: Accessibility labels could lose context after translation.
   - **Mitigation**: Reused existing common actions where possible and added focused common keys for search, notifications, dismissal, reorder, and line removal.

## Decisions

1. Keep literal examples such as `KSh 1,500.00`, `A4`, and `Letter` unchanged because they are product examples or standard paper-size labels.
2. Keep user-entered record values untranslated.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
