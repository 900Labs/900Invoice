# Sprint 067 - Release Gate First Tag Readiness

Status: Completed
Date: 2026-06-12
Owner: 900 Labs

## Context

The first `v1.0.0` release tag has no previous version tag to use as a release-governance diff base. The release workflow therefore falls back to the repository root commit, which includes historical sprint records created before maintainer checklist completion blocks were enforced.

The release dry run also showed that GitHub's default workflow token cannot read branch-protection settings from the Release Gate job. The governance audit workflow already supports a `GH_ADMIN_TOKEN` fallback for this same policy check.

## Goals

1. Make the first version tag pass the same release-governance checks as later tags.
2. Keep historical sprint records compatible with current checklist-completion enforcement.
3. Align Release Gate token handling with the governance audit workflow.
4. Document the release secret needed for strict branch-protection verification.

## Deliverables

1. Added `MAINTAINER_CHECKLIST_COMPLETION` blocks to Sprint 001 through Sprint 019 records.
2. Updated `.github/workflows/release.yml` to prefer `GH_ADMIN_TOKEN` and fall back to `github.token`.
3. Updated `docs/RELEASE.md` to document the `GH_ADMIN_TOKEN` requirement for strict repository-policy verification.
4. Added this Sprint 067 release-gate readiness record.

## Validation

Run from the repository root:

```bash
STRICT=1 GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main
REPORT_JSON_PATH=/tmp/release-governance-diff-context-v1.0.0-first-tag.json ./scripts/verify-governance-sprint-checklist.sh 1d17e7f3971867652f622df1a6119693fb6db14e HEAD
./scripts/verify-governance-trace-json.sh /tmp/release-governance-diff-context-v1.0.0-first-tag.json
./scripts/verify-api-doc-commands.sh
```

Expected result: repository policy passes with a token that can read branch protection; the first-tag governance diff passes and emits schema-valid JSON; API docs remain in sync.

## Decisions

1. Preserve historical sprint content and add only the current checklist-completion block.
2. Reuse the existing `GH_ADMIN_TOKEN` secret name already documented for governance audit workflows.
3. Keep the release workflow strict; token configuration is the maintainer responsibility rather than silently weakening branch-protection verification.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile selected or explicitly unchanged: `solo`.
- [x] Branch protection expectations reviewed against `docs/BRANCH_PROTECTION.md`.
- [x] Maintainer handoff expectations reviewed against `docs/MAINTAINER_CHECKLIST.md`.
- [x] Release or validation impact documented in this sprint record.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
