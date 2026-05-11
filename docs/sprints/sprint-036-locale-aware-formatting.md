# Sprint 036: Locale-Aware Formatting

- **Sprint ID**: `036`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README promises locale-aware date and number formatting. The app translated UI strings and supported RTL, but currency formatting still used `en-US`, month-name dates used hardcoded English month names, and saved locale preferences were not reapplied on startup.

## Goals

1. Centralize locale-to-Intl formatting preferences.
2. Make currency, month-name date, and relative-time formatting use the active app locale.
3. Reapply persisted locale settings during startup and persist locale changes from the top-bar language picker.

## In Scope

1. New `src/utils/locale.ts` formatter locale mapping.
2. Locale-aware `formatCurrency`, `formatDate`, and `timeAgo`.
3. Startup locale reapplication from settings.
4. Top-bar locale persistence.
5. I18N documentation, changelog, and sprint record updates.

## Out of Scope

1. Full RTL PDF text shaping.
2. Locale-specific parsing of typed numeric input.
3. Per-locale default date-format migration.

## Deliverables

1. Shared locale formatter helper.
2. Currency/date utility updates backed by `Intl`.
3. Persisted locale startup behavior.
4. Updated I18N documentation matching the live utility surface.

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

1. **Risk**: Locale formatting could drift between app surfaces.
   - **Mitigation**: Currency and date utilities now resolve their Intl locales through one helper.
2. **Risk**: Saved locale preferences could disagree with the active translation bundle.
   - **Mitigation**: `loadSettings()` reapplies the saved locale after settings load, and top-bar locale changes persist the setting.

## Decisions

1. Keep currency symbols from the existing app currency config while using Intl for numeric separators and digits.
2. Preserve explicit user-selected date patterns; only month-name and relative-time formatting depend on locale.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
