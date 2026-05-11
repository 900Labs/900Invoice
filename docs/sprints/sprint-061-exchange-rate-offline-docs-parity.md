# Sprint 061: Exchange Rate Offline Docs Parity

- **Sprint ID**: `061`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README offline claim is supported by the current source: the app has no shipped runtime internet request path. However, ADR 004 still described exchange-rate updates as an optional internet function, and the exchange-rate upsert command comment referred to an external frontend API call. That wording implied a shipped online rate updater that does not currently exist.

The unreleased changelog also retained a stale Senegal tax-country note after Sprint 059 restored Senegal to match the active `SN` seed rate.

## Goals

1. Align offline-first architecture notes with the current no-runtime-network implementation.
2. Keep exchange-rate docs precise about seeded defaults, cached rows, and future refresh tooling.
3. Remove the stale changelog contradiction around Senegal seed coverage.

## In Scope

1. `docs/adr/004-offline-first-architecture.md` network and exchange-rate wording.
2. `src-tauri/src/commands/exchange_rates.rs` command comment.
3. Changelog and sprint documentation.

## Out of Scope

1. Adding an online exchange-rate provider.
2. Changing exchange-rate seed data or snapshot behavior.
3. Changing the exchange-rate command contract.

## Deliverables

1. ADR 004 states that no shipped application workflow performs internet requests.
2. ADR 004 describes exchange rates as offline seeded defaults or externally supplied cached rows.
3. The exchange-rate upsert command comment no longer implies a current frontend API integration.
4. The changelog no longer says Senegal was unseeded.

## Validation

1. `rg -n -F "Exchange rate updates (falls back to cached rates; app includes sensible defaults)" docs/adr/004-offline-first-architecture.md` returned no matches.
2. `rg -n -F "No shipped application workflow performs internet requests" docs/adr/004-offline-first-architecture.md` confirmed the corrected ADR wording.
3. `rg -n -F "e.g. from an external API call on the frontend" src-tauri/src/commands/exchange_rates.rs` returned no matches.
4. `rg -n -F "unseeded Senegal" CHANGELOG.md` returned no matches.
5. `rg -n -i -e 'fetch\\(' -e 'reqwest' -e '@tauri-apps/plugin-http' -e 'window\\.fetch' src src-tauri/src` returned no matches.
6. `./scripts/verify-api-doc-commands.sh` passed.
7. `npm run check` passed.
8. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
9. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Readers may confuse cached exchange-rate support with live online updates.
   - **Mitigation**: ADR 004 now distinguishes shipped offline cache behavior from future import or online refresh tooling.
2. **Risk**: Future online exchange-rate work could accidentally become a core dependency.
   - **Mitigation**: ADR 004 keeps the rule that future online features must gracefully degrade and never block core invoicing workflows.

## Decisions

1. Correct documentation and comments rather than adding a network provider in a docs-parity sprint.
2. Keep exchange-rate cache and snapshot behavior unchanged.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
