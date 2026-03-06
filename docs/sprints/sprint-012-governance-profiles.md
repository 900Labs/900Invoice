# Sprint 012: Governance Profile Presets

- **Sprint ID**: `012`
- **Date**: 2026-03-06
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Governance controls were enforceable but required manual variable tuning per repository. Sprint 011 follow-ups called for profile presets (`solo`, `small-team`, `enterprise`) with explicit, reusable contracts.

## Goals

1. Define profile-based governance defaults for common maintainer models.
2. Keep policy application and verification behavior consistent through one resolver.
3. Ensure governance automation (audit + release gate) uses the same contract.

## In Scope

1. Add shared governance profile resolver for policy scripts.
2. Extend apply/verify scripts to enforce profile-driven review/code-owner/last-push settings.
3. Update governance and release workflows to consume profile variables.
4. Document profile matrix and automation variable contract.

## Out of Scope

1. Product runtime changes.
2. Branch-protection API surface expansion beyond profile matrix.
3. External notification integrations.

## Deliverables

1. Added `scripts/governance-profile-env.sh` shared resolver:
   - profiles: `solo`, `small-team`, `enterprise`
   - supports explicit env overrides
   - validates integer/boolean contracts
2. Updated `scripts/apply-repo-policy.sh`:
   - sources shared resolver
   - applies `required_approving_review_count`, `require_code_owner_reviews`, `require_last_push_approval`
3. Updated `scripts/verify-repo-policy.sh`:
   - sources shared resolver
   - verifies exact values for review count, code-owner requirement, and last-push approval
4. Updated `.github/workflows/governance-audit.yml`:
   - added `governance_profile` dispatch input
   - supports repo variable defaults + optional review-count override
   - incident payload now includes expected profile/override metadata
5. Updated `.github/workflows/release.yml`:
   - strict policy verification now consumes governance profile variables
6. Updated docs:
   - `docs/BRANCH_PROTECTION.md`
   - `docs/GOVERNANCE_AUDIT.md`
   - `docs/RELEASE.md`
   - `CHANGELOG.md`

## Validation

1. `bash -n scripts/governance-profile-env.sh scripts/apply-repo-policy.sh scripts/verify-repo-policy.sh` passed.
2. `ruby -ryaml -e 'Dir[".github/workflows/*.yml"].each{|f| YAML.safe_load(File.read(f)); puts "ok #{f}" }'` passed.
3. `./scripts/verify-api-doc-commands.sh` passed (`60` commands).
4. `npm run check` passed (`0` errors, `0` warnings).
5. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.

## Risks and Mitigations

1. **Risk**: profile defaults diverge from live branch protection over time.
   - **Mitigation**: same resolver contract used by apply, verify, governance audit, and release gate checks.
2. **Risk**: invalid custom overrides break automation.
   - **Mitigation**: strict integer/boolean validation with explicit failure messages.

## Decisions

1. Keep `solo` as the default profile for autonomous maintainer workflows.
2. Encode team-scale governance through profile presets first, then allow explicit env overrides.

## Follow-Ups

1. Add profile-specific docs examples for onboarding templates in CONTRIBUTING/maintainer guide.
2. Add optional profile assertion output artifact in governance-audit workflow for easier incident triage.
