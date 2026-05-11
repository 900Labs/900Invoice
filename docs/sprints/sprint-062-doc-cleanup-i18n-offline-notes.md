# Sprint 062: Documentation Cleanup for I18N and Offline Notes

- **Sprint ID**: `062`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

A docs cleanup audit found two active documentation drifts:

1. `CONTRIBUTING.md` still instructed language contributors to register locales in `src/i18n/index.ts` using `SUPPORTED_LANGUAGES`, but the current runtime registry lives in `src/stores/i18nStore.ts` as `SUPPORTED_LOCALES`.
2. ADR 004's contributor note still used "exchange rate updates" as the optional network example after the current app had been clarified as having no shipped runtime internet workflow.

Ignored local `.DS_Store` files were also present in the working directory, but they are already covered by `.gitignore` and are not tracked.

## Goals

1. Align language-contribution docs with the current i18n store implementation.
2. Keep offline-first contributor notes consistent with the no-runtime-network implementation.
3. Keep the cleanup docs-only and low-risk.

## In Scope

1. `CONTRIBUTING.md` language contribution instructions.
2. `docs/adr/004-offline-first-architecture.md` contributor note.
3. Changelog and sprint documentation.

## Out of Scope

1. Changing locale runtime behavior.
2. Adding or removing translations.
3. Removing ignored local filesystem artifacts.

## Deliverables

1. `CONTRIBUTING.md` references `src/stores/i18nStore.ts`, `loadTranslations()`, `SUPPORTED_LOCALES`, nested JSON, `src/utils/locale.ts`, and current RTL handling.
2. Obsolete `SUPPORTED_LANGUAGES`, `src/i18n/index.ts`, and Rust settings-test instructions are removed from active contribution docs.
3. ADR 004 uses future external rate refresh as the optional-network example.

## Validation

1. `rg -n -F -e 'SUPPORTED_LANGUAGES' -e 'src/i18n/index.ts' CONTRIBUTING.md README.md docs --glob '!docs/sprints/**'` returned no matches.
2. `rg -n -F 'future external rate refresh' docs/adr/004-offline-first-architecture.md` confirmed the updated offline contributor note.
3. `rg -n -F -e 'SUPPORTED_LOCALES' -e 'src/stores/i18nStore.ts' -e 'loadTranslations()' CONTRIBUTING.md` confirmed current i18n references.
4. `./scripts/verify-api-doc-commands.sh` passed.
5. `npm run check` passed.
6. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
7. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Contributors may miss the more detailed i18n guide.
   - **Mitigation**: `CONTRIBUTING.md` now mirrors the current implementation and still points contributors to `docs/I18N.md`.
2. **Risk**: Optional online rate refresh language may be read as a current feature.
   - **Mitigation**: ADR 004 explicitly frames it as future external refresh tooling.

## Decisions

1. Keep ignored `.DS_Store` files out of the PR because they are not tracked and `.gitignore` already covers them.
2. Remove the stale Rust settings-test instruction because there is no current language-code validation test in `src-tauri/src/commands/settings.rs`.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
