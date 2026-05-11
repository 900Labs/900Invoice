# Sprint 059: Tax Country Seed Docs Parity

- **Sprint ID**: `059`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README pre-configured tax-rate country list omitted Senegal, but the current database seed data includes an active Senegal `SN` VAT rate through `tax-xof-vat`. That made the README understate the current tax engine seed coverage.

## Goals

1. Align the README tax country list with the current seed data.
2. Record validation evidence for the active `SN` seed rate.
3. Keep the change limited to documentation parity.

## In Scope

1. README tax engine country list.
2. Changelog and sprint documentation.

## Out of Scope

1. Changing seeded tax rates.
2. Adding or removing country support.
3. Tax-rate UI behavior.

## Deliverables

1. README lists Senegal among pre-configured tax-rate countries.
2. Sprint record captures the seed-data evidence used for the correction.

## Validation

1. `rg -n -F "country_code, is_default, is_withholding" src-tauri/src/db/migrations.rs` confirmed the seed insert target columns.
2. `rg -n -F "'tax-xof-vat',   'VAT',     'VAT @ 18%',      1800, 'SN'" src-tauri/src/db/migrations.rs` confirmed the active Senegal seed rate.
3. `rg -n -F "Pre-configured rates for Kenya, Nigeria, South Africa, India, Ghana, Tanzania, Uganda, Senegal" README.md` confirmed README parity.
4. `./scripts/verify-api-doc-commands.sh` passed.
5. `npm run check` passed.
6. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
7. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: The country list could drift again if seed rows change.
   - **Mitigation**: This sprint records the exact seed-data checks used to validate the README list.

## Decisions

1. Restore Senegal to the README because the current implementation already ships an `SN` seed row.
2. Avoid modifying tax seed data in a documentation parity sprint.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
