# Sprint 016: Release Gate Governance Checklist Parity

- **Sprint ID**: `016`
- **Date**: 2026-03-06
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 015 enforced governance checklist references in pull-request CI, but release workflows still lacked mirrored enforcement outside PR context.

## Goals

1. Enforce governance sprint-checklist verification in release gate workflows.
2. Reuse existing governance checklist enforcement logic instead of duplicating policy rules.
3. Document release-side governance parity behavior.

## In Scope

1. Add governance checklist enforcement step to `.github/workflows/release.yml`.
2. Resolve release diff base (`previous tag` for tag releases, `origin/main` for manual runs).
3. Update release runbook and changelog.
4. Add sprint record.

## Out of Scope

1. Runtime product behavior changes.
2. Branch protection policy changes.
3. External alert/notification integrations.

## Deliverables

1. Updated `.github/workflows/release.yml`:
   - added `Resolve governance checklist diff base` step
   - computes base from previous tag (or repository root when no previous tag exists)
   - uses `origin/main` as base for manual release runs
   - added `Verify governance sprint checklist reference` step using existing script
2. Updated `docs/RELEASE.md`:
   - documented governance checklist parity in release model and pre-tag checklist
3. Updated `CHANGELOG.md` in `[Unreleased]`.
4. Added sprint record `docs/sprints/sprint-016-release-gate-governance-parity.md`.

## Validation

1. `bash -n scripts/verify-governance-sprint-checklist.sh` passed.
2. `./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
3. `ruby -ryaml -e 'Dir[".github/workflows/*.yml"].each{|f| YAML.safe_load(File.read(f)); puts "ok #{f}" }'` passed.
4. `./scripts/verify-api-doc-commands.sh` passed (`60` commands).
5. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
6. `npm run check` timed out in this local environment; CI `Quality Gate` remains required and authoritative.

## Risks and Mitigations

1. **Risk**: release workflow false failures due diff-base ambiguity.
   - **Mitigation**: explicit base resolution rules for tag/manual contexts with deterministic fallbacks.
2. **Risk**: governance enforcement drift between CI and release workflows.
   - **Mitigation**: release gate reuses `scripts/verify-governance-sprint-checklist.sh` directly.

## Decisions

1. Keep one enforcement script and invoke it from both CI and release gate.
2. Resolve previous-tag scope in workflow rather than embedding tag logic in the script.

## Follow-Ups

1. Add workflow artifact/summary output that records selected governance diff base for release audit traceability.
2. Consider optional strict mode to require checklist references in all sprint docs within release diff, not just at least one.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.
