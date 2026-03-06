# Sprint 018: Governance Trace Pack (JSON + Strict Mode)

- **Sprint ID**: `018`
- **Date**: 2026-03-06
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 017 delivered text-based governance diff traceability in release workflows. The next efficiency pack combined two follow-ups:

1. machine-readable governance trace output
2. optional strict enforcement mode requiring checklist references across all changed sprint docs

## Goals

1. Add JSON governance trace output for release analytics.
2. Add optional strict governance checklist enforcement mode.
3. Keep CI and release workflows aligned with the same enforcement contract.

## In Scope

1. Extend governance checklist script with JSON reporting support.
2. Add strict mode enforcement toggle (`STRICT_SPRINT_DOC_REFERENCE`).
3. Wire strict mode into CI and release workflows via repository variables.
4. Publish JSON trace artifact from release gate.
5. Update release/maintainer docs and changelog.

## Out of Scope

1. Runtime product behavior changes.
2. Branch protection policy threshold changes.
3. External notification integrations.

## Deliverables

1. Updated `scripts/verify-governance-sprint-checklist.sh`:
   - added `REPORT_JSON_PATH` output support
   - emits machine-readable JSON payload with evaluated files, sprint-doc matches, result, and reason
   - added `STRICT_SPRINT_DOC_REFERENCE` mode (`0`/`1`, accepts boolean aliases)
   - in strict mode, all changed sprint docs must reference `docs/MAINTAINER_CHECKLIST.md`
2. Updated `.github/workflows/ci.yml`:
   - passes `STRICT_SPRINT_DOC_REFERENCE` from repository variables for PR governance enforcement
3. Updated `.github/workflows/release.yml`:
   - passes `STRICT_SPRINT_DOC_REFERENCE` from repository variables
   - emits both text and JSON governance diff context files
   - uploads JSON trace in release governance artifact
   - adds JSON trace note to workflow summary
4. Updated docs:
   - `docs/RELEASE.md`
   - `docs/MAINTAINER_CHECKLIST.md`
   - `CHANGELOG.md`

## Validation

1. `bash -n scripts/verify-governance-sprint-checklist.sh` passed.
2. `REPORT_PATH=/tmp/release-governance-diff-context.txt REPORT_JSON_PATH=/tmp/release-governance-diff-context.json ./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
3. `ruby -ryaml -e 'Dir[".github/workflows/*.yml"].each{|f| YAML.safe_load(File.read(f)); puts "ok #{f}" }'` passed.
4. `./scripts/verify-api-doc-commands.sh` passed (`60` commands).
5. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
6. `npm run check` timed out in this local environment; CI `Quality Gate` remains required and authoritative.

## Risks and Mitigations

1. **Risk**: strict mode causes unexpected failures on existing governance PR habits.
   - **Mitigation**: strict mode is optional (`STRICT_SPRINT_DOC_REFERENCE` defaults to `0`).
2. **Risk**: report drift between text and JSON payloads.
   - **Mitigation**: both outputs are generated from the same script state during one execution.

## Decisions

1. Keep strict mode opt-in through repository variable configuration.
2. Keep JSON payload generation in the existing script to avoid duplicate workflow logic.

## Follow-Ups

1. Add JSON schema validation for governance trace payload in CI.
2. Add optional archival retention policy guidance for governance trace artifacts.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.
