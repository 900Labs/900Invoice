# Sprint 011: Governance Incident Routing

- **Sprint ID**: `011`
- **Date**: 2026-03-05
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 010 delivered scheduled governance drift detection but still relied on maintainers actively watching Actions runs. The next gap was automated incident routing when a governance audit fails.

## Goals

1. Route governance-audit failures to a durable collaboration channel.
2. Avoid duplicate incident tickets while preserving failure history.
3. Keep the workflow lightweight and compatible with current repository governance.

## In Scope

1. Add automated issue creation/update on governance-audit failure.
2. Update runbook documentation for incident behavior.
3. Update changelog and sprint record.

## Out of Scope

1. External chat/email/pager integrations.
2. Product runtime changes.
3. Branch-protection policy redesign.

## Deliverables

1. Updated `.github/workflows/governance-audit.yml`:
   - granted `issues: write` permission
   - added step IDs for check outcome capture
   - added `if: failure()` incident step using `actions/github-script@v7`
   - implemented create-or-comment behavior for issue title `Governance audit failure: policy drift detected`
   - standardized incident labels: `governance-audit`, `incident`, `automation`
2. Updated `docs/GOVERNANCE_AUDIT.md`:
   - added incident routing section with behavior and payload details
3. Updated `CHANGELOG.md` in `[Unreleased]`.

## Validation

1. `ruby -ryaml -e 'Dir[".github/workflows/*.yml"].each{|f| YAML.safe_load(File.read(f)); puts "ok #{f}" }'` passed.
2. `./scripts/verify-api-doc-commands.sh` passed (`60` commands).
3. `npm run check` passed (`0` errors, `0` warnings).
4. `STRICT=1 ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.

## Risks and Mitigations

1. **Risk**: incident spam from repeated failures.
   - **Mitigation**: workflow reuses a single open issue title and appends comments for subsequent failures.
2. **Risk**: missing issue permissions in workflow token context.
   - **Mitigation**: explicit `issues: write` permission in workflow.

## Decisions

1. Route failures to GitHub Issues first for transparent, open-source incident tracking.
2. Defer external notification channels to a future sprint.

## Follow-Ups

1. Add optional external notifications (email/chat/webhook) fed from governance incident issue events.
2. Define governance profile presets (`solo`, `small-team`, `enterprise`) with validated environment-variable contracts.
