# Sprint 022: Governance Artifact Retention Policy

- **Sprint ID**: `022`
- **Date**: 2026-03-07
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 018 and Sprint 021 established governance trace artifacts and schema validation. A remaining follow-up was explicit retention guidance so artifact lifecycle stays predictable and storage usage remains controlled.

## Goals

1. Add configurable governance artifact retention policy for governance workflows.
2. Enforce safe retention-range validation in automation.
3. Document retention policy for maintainers.

## In Scope

1. Add retention-days variable support in governance workflows.
2. Validate retention-days value range in workflow runtime.
3. Apply retention policy to governance artifacts.
4. Update governance/release/maintainer docs and changelog.

## Out of Scope

1. Product runtime behavior changes.
2. Governance trace payload contract changes.
3. Third-party archival/storage integrations.

## Deliverables

1. Updated `.github/workflows/release.yml`:
   - added `GOVERNANCE_ARTIFACT_RETENTION_DAYS` env (default `30`)
   - added retention policy validation step (`1`-`90`)
   - applied retention-days to `release-governance-diff-context-<tag>` artifact upload
2. Updated `.github/workflows/governance-audit.yml`:
   - added `GOVERNANCE_ARTIFACT_RETENTION_DAYS` env (default `30`)
   - added retention policy validation step (`1`-`90`)
   - applied retention-days to governance profile assertion artifact upload
3. Updated docs:
   - `docs/RELEASE.md`
   - `docs/GOVERNANCE_AUDIT.md`
   - `docs/MAINTAINER_CHECKLIST.md`
   - `CHANGELOG.md`

## Validation

1. `ruby -ryaml -e 'YAML.load_file(".github/workflows/release.yml"); YAML.load_file(".github/workflows/governance-audit.yml"); puts "ok"'` passed.
2. `./scripts/verify-api-doc-commands.sh` passed.
3. `./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
4. `REPORT_JSON_PATH=/tmp/release-governance-diff-context.json ./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
5. `./scripts/verify-governance-trace-json.sh /tmp/release-governance-diff-context.json` passed.
6. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
7. `npm run check` remains environment-sensitive locally due esbuild platform mismatch in current `node_modules`; CI `Quality Gate` remains authoritative.

## Risks and Mitigations

1. **Risk**: invalid retention value blocks governance workflows.
   - **Mitigation**: explicit validation step with clear error message and documented valid range.
2. **Risk**: retention policy misunderstanding across maintainers.
   - **Mitigation**: added variable guidance in release/governance runbooks and maintainer checklist.

## Decisions

1. Use one shared governance artifact retention variable for both release and governance-audit workflows.
2. Set default retention to `30` days for governance artifacts.
3. Enforce `1`-`90` range in workflow runtime to prevent invalid artifact upload configuration.

## Follow-Ups

1. Add optional per-workflow retention overrides if governance and release teams need different retention windows.
2. Add periodic governance artifact inventory reporting for retention policy audits.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
