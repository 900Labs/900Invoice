# Sprint 019: Governance Incident External Notifications

- **Sprint ID**: `019`
- **Date**: 2026-03-06
- **Status**: Completed
- **Owner**: 900 Labs

## Context

Sprint 011 established governance incident routing to GitHub Issues. A deferred follow-up remained: optional external notifications for maintainers who also rely on chat/email escalation channels.

## Goals

1. Add optional external notification delivery for governance-audit incidents.
2. Keep GitHub Issues as the default transparent incident channel.
3. Keep implementation lightweight and compatible with minimal CI environments.

## In Scope

1. Extend governance-audit workflow with optional webhook notification step.
2. Reuse incident issue metadata from the existing failure-routing step.
3. Document secret configuration and payload contract for maintainers.
4. Update changelog and sprint records.

## Out of Scope

1. Provider-specific integrations (Slack/Teams/email APIs) in repository code.
2. Message templating for multiple third-party formats.
3. Runtime product application behavior changes.

## Deliverables

1. Updated `.github/workflows/governance-audit.yml`:
   - added `id: incident_issue` and outputs for incident issue URL/number/action
   - added optional `Notify external governance incident webhook` step
   - webhook step posts JSON payload to `GOVERNANCE_INCIDENT_WEBHOOK_URL`
   - supports optional bearer token via `GOVERNANCE_INCIDENT_WEBHOOK_TOKEN`
2. Updated `docs/GOVERNANCE_AUDIT.md`:
   - added optional secret configuration
   - documented notification behavior and payload fields
3. Updated `docs/MAINTAINER_CHECKLIST.md`:
   - added governance-incident checklist items for webhook secret validation
4. Updated `CHANGELOG.md` with sprint outcomes.

## Validation

1. `ruby -ryaml -e 'YAML.load_file(".github/workflows/governance-audit.yml"); puts "ok"'` passed.
2. `./scripts/verify-api-doc-commands.sh` passed.
3. `STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main` passed.
4. `./scripts/verify-governance-sprint-checklist.sh origin/main HEAD` passed.
5. `npm run check` remains environment-sensitive locally; CI `Quality Gate` remains the release authority.

## Risks and Mitigations

1. **Risk**: webhook endpoint outage during incidents.
   - **Mitigation**: GitHub Issue routing remains the primary/default incident channel.
2. **Risk**: secret misconfiguration causing notification failures.
   - **Mitigation**: explicit maintainer checklist entries and runbook configuration section.
3. **Risk**: introducing heavy dependencies in CI.
   - **Mitigation**: implementation uses built-in shell + `curl` only.

## Decisions

1. Keep external notifications optional and secret-driven.
2. Keep notification payload generic JSON so maintainers can route to chat/email gateways without repository code changes.
3. Run external notification only after incident issue routing succeeds.

## Follow-Ups

1. Add optional retry/backoff policy for external webhook delivery.
2. Add optional signed request verification (HMAC) for receivers requiring stronger authenticity checks.

## Maintainer Checklist Reference

This sprint follows and references: `docs/MAINTAINER_CHECKLIST.md`.
