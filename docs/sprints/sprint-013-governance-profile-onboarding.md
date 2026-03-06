# Sprint 013: Governance Profile Onboarding

- **Sprint ID**: `013`
- **Date**: 2026-03-06
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 012 introduced profile presets (`solo`, `small-team`, `enterprise`) but follow-ups remained for maintainer onboarding examples and easier triage output during governance-audit incidents.

## Goals

1. Add maintainer-facing onboarding examples for governance profile adoption.
2. Improve governance-audit triage by publishing resolved profile assertions as run artifacts.
3. Keep strict policy verification and incident routing aligned with resolved profile values.

## In Scope

1. Update governance-audit workflow to resolve profile inputs once and export assertion data.
2. Upload governance profile assertion artifact for every audit run.
3. Add maintainer onboarding profile examples in `CONTRIBUTING.md`.
4. Update governance docs/changelog and record sprint outcome.

## Out of Scope

1. Runtime product behavior changes.
2. External chat/email notification integrations.
3. Additional branch-protection policy redesign.

## Deliverables

1. Updated `.github/workflows/governance-audit.yml`:
   - added `Resolve governance profile inputs` step
   - outputs resolved profile and policy fields
   - uploads `governance-profile-assertion-<run_id>` artifact from `.tmp/governance-profile-assertion.txt`
   - incident issue metadata now uses resolved profile and includes code-owner/last-push expectations
2. Updated `docs/GOVERNANCE_AUDIT.md`:
   - documented profile assertion artifact behavior and fields
3. Updated `CONTRIBUTING.md`:
   - added `Governance Profile Onboarding (Maintainers)` section
   - included apply/verify command examples for `solo`, `small-team`, and `enterprise`
4. Updated `CHANGELOG.md` in `[Unreleased]`.

## Validation

1. `bash -n scripts/governance-profile-env.sh scripts/apply-repo-policy.sh scripts/verify-repo-policy.sh` passed.
2. `ruby -ryaml -e 'Dir[".github/workflows/*.yml"].each{|f| YAML.safe_load(File.read(f)); puts "ok #{f}" }'` passed.
3. `./scripts/verify-api-doc-commands.sh` passed (`60` commands).
4. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
5. `npm run check` timed out in this local environment; CI `Quality Gate` remains required and authoritative.

## Risks and Mitigations

1. **Risk**: governance incident triage ambiguity about which profile was evaluated.
   - **Mitigation**: assertion artifact captures resolved profile + overrides per run.
2. **Risk**: maintainers misconfigure profile variables during onboarding.
   - **Mitigation**: explicit profile examples and variable contract guidance in contributor docs.

## Decisions

1. Resolve governance profile inputs once per audit run and reuse across checks/incidents.
2. Keep profile assertions as lightweight workflow artifacts instead of introducing external storage.

## Follow-Ups

1. Add a direct link to the assertion artifact in incident issue comments when platform support allows stable artifact URLs.
2. Add a maintainer checklist template that includes governance profile selection and variable verification.
