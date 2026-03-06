# Sprint 021: Governance Trace Schema Validation

- **Sprint ID**: `021`
- **Date**: 2026-03-07
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 018 introduced machine-readable governance diff trace JSON output and identified a follow-up: enforce schema validation for that payload in CI so contract drift is caught before merge/release.

## Goals

1. Define a formal schema contract for governance diff trace JSON.
2. Enforce schema validation in CI and release workflows.
3. Keep validation lightweight and compatible with constrained environments.

## In Scope

1. Add governance diff trace JSON schema file.
2. Add validator script for governance trace JSON payloads.
3. Wire validation into CI (PRs) and release gate workflow.
4. Update quality/release documentation and changelog.

## Out of Scope

1. Product runtime behavior changes.
2. Governance incident routing channel changes.
3. Archival retention policy changes for governance artifacts.

## Deliverables

1. Added `docs/schemas/governance-diff-trace.schema.json`.
2. Added `scripts/verify-governance-trace-json.sh`:
   - validates trace JSON structure/types/enums
   - rejects unexpected top-level keys
   - validates both schema and payload are parseable JSON
3. Updated `.github/workflows/ci.yml`:
   - PR governance step emits `.tmp/ci-governance-diff-context.json`
   - added governance trace schema validation step
4. Updated `.github/workflows/release.yml`:
   - added governance trace schema validation step before publishing summary/artifacts
5. Updated docs:
   - `docs/RELEASE.md`
   - `docs/QUALITY_GATE.md`
   - `CHANGELOG.md`

## Validation

1. `bash -n scripts/verify-governance-trace-json.sh` passed.
2. `ruby -ryaml -e 'YAML.load_file(".github/workflows/ci.yml"); YAML.load_file(".github/workflows/release.yml"); puts "ok"'` passed.
3. `REPORT_JSON_PATH=/tmp/release-governance-diff-context.json ./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
4. `./scripts/verify-governance-trace-json.sh /tmp/release-governance-diff-context.json` passed.
5. `./scripts/verify-api-doc-commands.sh` passed.
6. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
7. `npm run check` remains environment-sensitive locally; CI `Quality Gate` remains required and authoritative.

## Risks and Mitigations

1. **Risk**: schema and validator drift.
   - **Mitigation**: validator validates against explicit key/type contract and fails on unknown top-level keys.
2. **Risk**: additional CI overhead.
   - **Mitigation**: validation is shell + `jq` only and runs on small JSON payloads.

## Decisions

1. Keep validator dependency footprint minimal (`bash` + `jq`).
2. Enforce schema validation in both PR CI and release gate for parity.
3. Keep schema as a versioned repository artifact under `docs/schemas/`.

## Follow-Ups

1. Add optional schema-version field if governance trace payloads evolve across major versions.
2. Add artifact retention policy guidance for governance trace outputs.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
