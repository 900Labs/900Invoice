# Sprint 060: Node Engine Docs Parity

- **Sprint ID**: `060`
- **Date**: 2026-05-11
- **Status**: Completed
- **Owner**: 900 Labs

## Context

The README and deployment guide advertised Node.js 18+ as sufficient for source builds, but the locked frontend dependency graph includes Svelte/Vite tooling that declares `^20.19 || ^22.12 || >=24`. That made the source-build prerequisite lower than the actual install/build engine range.

## Goals

1. Document the Node.js source-build prerequisite that matches the locked frontend dependency graph.
2. Pin the root package engine so npm exposes the requirement.
3. Keep README, deployment docs, `package.json`, and `package-lock.json` aligned.

## In Scope

1. README source-build prerequisite.
2. Deployment guide prerequisite.
3. Root package engine metadata.
4. Changelog and sprint documentation.

## Out of Scope

1. Downgrading frontend dependencies for Node 18 compatibility.
2. Changing GitHub Actions runtime versions.
3. Release binary packaging.

## Deliverables

1. README and deployment docs advertise Node.js 20.19+, 22.12+, or 24+.
2. `package.json` and root `package-lock.json` metadata declare `^20.19.0 || ^22.12.0 || >=24.0.0`.
3. Validation records the locked dependency engine range that drove the correction.

## Validation

1. `rg -n -F 'Node.js 18+' README.md docs/DEPLOYMENT.md` returned no matches.
2. `rg -n -F -e 'Node.js 20.19+, 22.12+, or 24+' README.md docs/DEPLOYMENT.md` confirmed the updated docs.
3. `node -e` package metadata checks confirmed root `package.json`, root `package-lock.json`, `@sveltejs/vite-plugin-svelte`, and `@sveltejs/vite-plugin-svelte-inspector` engine ranges.
4. `./scripts/verify-api-doc-commands.sh` passed.
5. `npm run check` passed.
6. `SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh` passed.
7. `git diff --check` passed.

## Risks and Mitigations

1. **Risk**: Contributors on Node 18 may only discover the requirement after npm emits engine warnings or failures.
   - **Mitigation**: The README, deployment guide, and root package metadata now expose the same engine range.
2. **Risk**: Future frontend dependency updates may change the effective engine floor.
   - **Mitigation**: Validation records the dependency-engine check used to compare docs with the locked graph.

## Decisions

1. Raise the documented prerequisite instead of downgrading the frontend dependency graph.
2. Use the exact root engine range `^20.19.0 || ^22.12.0 || >=24.0.0` to match current Svelte/Vite tooling.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
