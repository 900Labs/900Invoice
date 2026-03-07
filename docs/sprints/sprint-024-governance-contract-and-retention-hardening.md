# Sprint 024: Governance Contract and Retention Hardening Bundle

- **Sprint ID**: `024`
- **Date**: 2026-03-07
- **Status**: Completed
- **Owner**: 900 Labs

## Context

After Sprint 023 webhook hardening, remaining governance hardening follow-ups were tightly related and efficient to combine:

1. checklist completion block lint hardening
2. governance trace schema versioning and compatibility rules
3. per-workflow retention overrides with periodic governance artifact inventory reporting

## Goals

1. Make checklist completion block validation stricter and less error-prone.
2. Add explicit governance trace schema versioning with backward compatibility rules.
3. Improve governance artifact lifecycle control and auditability across workflows.

## In Scope

1. Enhance `scripts/verify-governance-sprint-checklist.sh` completion-block lint behavior.
2. Add schema-versioned governance trace contract and validator compatibility handling.
3. Add per-workflow retention override resolution in governance-audit and release workflows.
4. Add scheduled governance artifact inventory workflow and report artifacts.
5. Update governance/release/maintainer documentation and changelog.

## Out of Scope

1. Product runtime behavior changes.
2. Non-governance CI matrix expansion.
3. External storage integrations beyond GitHub artifact inventory.

## Deliverables

1. Updated `scripts/verify-governance-sprint-checklist.sh`:
   - JSON trace now includes `schema_version` (`1.1.0`)
   - completion-block lint now detects and fails malformed markers (duplicate/nested/unbalanced)
   - report output includes `sprint_docs_malformed_checklist_completion_block`
   - governance-impacting allowlist expanded for governance workflow/schema/validator files
2. Updated `docs/schemas/governance-diff-trace.schema.json`:
   - defines versioned compatibility contract (`1.0.0` + `1.1.0`)
3. Updated `scripts/verify-governance-trace-json.sh`:
   - supports schema-version compatibility rules
   - accepts legacy `1.0.0` payloads (including missing `schema_version`)
   - validates current `1.1.0` payloads (including malformed-block array)
4. Updated `.github/workflows/release.yml`:
   - release-specific governance retention override (`RELEASE_GOVERNANCE_ARTIFACT_RETENTION_DAYS`)
   - shared fallback to `GOVERNANCE_ARTIFACT_RETENTION_DAYS`
   - summary now reports retention value and source
5. Updated `.github/workflows/governance-audit.yml`:
   - governance-audit retention override (`GOVERNANCE_AUDIT_ARTIFACT_RETENTION_DAYS`)
   - shared fallback to `GOVERNANCE_ARTIFACT_RETENTION_DAYS`
   - summary includes retention metadata
6. Added `.github/workflows/governance-artifact-inventory.yml`:
   - weekly + manual governance artifact inventory reporting
   - uploads markdown/json inventory artifacts
   - supports override `GOVERNANCE_INVENTORY_ARTIFACT_RETENTION_DAYS`
7. Updated docs:
   - `docs/GOVERNANCE_AUDIT.md`
   - `docs/RELEASE.md`
   - `docs/MAINTAINER_CHECKLIST.md`
   - `CHANGELOG.md`

## Validation

1. `bash -n scripts/verify-governance-sprint-checklist.sh` passed.
2. `bash -n scripts/verify-governance-trace-json.sh` passed.
3. `ruby -ryaml -e 'YAML.load_file(".github/workflows/governance-audit.yml"); YAML.load_file(".github/workflows/release.yml"); YAML.load_file(".github/workflows/governance-artifact-inventory.yml"); puts "ok"'` passed.
4. `./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
5. `REPORT_JSON_PATH=/tmp/release-governance-diff-context.json ./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
6. `./scripts/verify-governance-trace-json.sh /tmp/release-governance-diff-context.json` passed.
7. `./scripts/verify-api-doc-commands.sh` passed.
8. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
9. `npm run check` fails locally due esbuild platform mismatch in current `node_modules`; CI `Quality Gate` remains authoritative.

## Risks and Mitigations

1. **Risk**: stricter checklist linting may initially fail contributor PRs.
   - **Mitigation**: explicit failure reasons and template-based block requirements remain documented.
2. **Risk**: schema-version transition ambiguity.
   - **Mitigation**: validator supports legacy (`1.0.0`/missing version) and current (`1.1.0`) contracts.
3. **Risk**: retention override misconfiguration.
   - **Mitigation**: each workflow resolves/validates bounded retention with clear source labeling.

## Decisions

1. Use `schema_version=1.1.0` for new governance trace payloads while preserving legacy compatibility.
2. Keep malformed block markers as an explicit failing category for maintainer actionability.
3. Keep one shared retention default with explicit per-workflow overrides.
4. Add native periodic inventory reporting via GitHub Actions instead of external systems.

## Follow-Ups

1. Add optional alerting thresholds for governance artifact inventory metrics (for example, high near-expiration counts).
2. Add optional checksum/signature metadata for inventory report artifacts if required by governance policy.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
