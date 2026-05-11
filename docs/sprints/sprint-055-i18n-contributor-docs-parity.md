# Sprint 055: I18N Contributor Docs Parity

- **Sprint ID**: `055`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The i18n guide described an older structure with `src/i18n/index.ts`, a `SUPPORTED_LANGUAGES` array, flat dotted JSON keys, and `$t()`. The current app uses nested JSON files in `src/i18n/`, loads them through `src/stores/i18nStore.ts`, exposes `t()`, and registers picker entries in `SUPPORTED_LOCALES`. RTL direction is handled in `setLocale()` rather than through a `dir` property on locale entries.

## Goals

1. Align the contributor guide with the current i18n store and file layout.
2. Show nested JSON examples that match the actual translation files.
3. Correct language registration and RTL instructions.

## In Scope

1. `docs/I18N.md` overview, file structure, key examples, registration steps, and RTL steps.
2. Changelog and sprint documentation.

## Out of Scope

1. Changing the i18n runtime implementation.
2. Adding a new language.
3. Adding generic RTL metadata to locale entries.

## Deliverables

1. The guide references `src/stores/i18nStore.ts`, `loadTranslations()`, `SUPPORTED_LOCALES`, and `t()`.
2. Translation examples use nested JSON objects and dot-path lookup examples.
3. RTL instructions tell contributors to update `setLocale()` for new RTL language codes.

## Validation

1. `rg -n -F -e 'src/i18n/index.ts' -e 'SUPPORTED_LANGUAGES' -e '$t()' -e "dir: 'rtl'" docs/I18N.md` returned no matches.
2. `rg -n -F -e '$t(' -e "from '../i18n'" -e 'invoice.items_count' -e 'invoice.status.draft' -e 'client.name_label' docs/I18N.md` returned no matches.
3. `rg -n -F -e 'src/stores/i18nStore.ts' -e 'SUPPORTED_LOCALES' -e 'loadTranslations()' -e 't()' docs/I18N.md` confirmed the current implementation names are documented.
4. `./scripts/verify-api-doc-commands.sh` passed.
5. `npm run check` passed.
6. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
7. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: The guide still requires contributors to understand a manual switch-based loader.
   - **Mitigation**: This sprint documents the current implementation exactly; a future runtime refactor can simplify registration.
2. **Risk**: New RTL languages need code changes in `setLocale()`.
   - **Mitigation**: The guide now calls out that requirement explicitly.

## Decisions

1. Correct the docs to match the current i18n store instead of introducing locale `dir` metadata in this sprint.
2. Keep nested JSON as the documented translation shape because it matches every current locale file.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
