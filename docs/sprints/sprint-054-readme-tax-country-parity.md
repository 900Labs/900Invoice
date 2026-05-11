# Sprint 054: README Tax Country Parity

- **Sprint ID**: `054`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README claimed pre-configured tax rates for Senegal. The current migration seed data includes tax rates for Kenya, Nigeria, South Africa, India, Ghana, Tanzania, and Uganda, but does not seed any Senegal tax-rate rows. Senegal remains available elsewhere as a client country option and XOF remains a supported currency, but the tax-rate claim overstated the seeded tax catalog.

## Goals

1. Align the README pre-configured tax-rate country list with seeded tax-rate data.
2. Avoid adding unverified tax-rate defaults without a current source.
3. Keep currency and client-country support claims unchanged.

## In Scope

1. README Tax Engine country list.
2. Changelog and sprint documentation.

## Out of Scope

1. Adding Senegal tax-rate seed data.
2. Changing currency support.
3. Changing client country options.

## Deliverables

1. README no longer claims pre-configured Senegal tax rates.
2. README tax-rate countries match the current migration seed set.

## Validation

1. `rg -n -F 'Pre-configured rates for Kenya, Nigeria, South Africa, India, Ghana, Tanzania, Uganda, Senegal' README.md` returned no matches.
2. `rg -n -F 'tax-sn' src-tauri/src/db/migrations.rs` returned no matches, confirming no Senegal seed rows exist.
3. `./scripts/verify-api-doc-commands.sh` passed.
4. `npm run check` passed.
5. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
6. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Users in Senegal may infer the app lacks Senegal support entirely.
   - **Mitigation**: This sprint only removes the tax-rate seed claim; Senegal remains available as a client country and XOF remains supported.
2. **Risk**: Senegal tax defaults may be desirable later.
   - **Mitigation**: A future tax-rate seed sprint should use current, verified tax sources before adding defaults.

## Decisions

1. Correct documentation instead of adding unverified tax defaults.
2. Keep the README country list scoped to seeded tax rates, not broader country or currency support.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
