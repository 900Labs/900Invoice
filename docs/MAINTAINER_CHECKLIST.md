# Maintainer Checklist

This checklist is for maintainers preparing policy updates, sprint merges, and releases.

Use it as the canonical governance handoff template.

---

## 1. Governance Profile Selection

Select one governance profile for the repository:

1. `solo`
   - `REQUIRED_APPROVING_REVIEW_COUNT=0`
   - `REQUIRE_CODE_OWNER_REVIEWS=false`
   - `REQUIRE_LAST_PUSH_APPROVAL=false`
2. `small-team`
   - `REQUIRED_APPROVING_REVIEW_COUNT=1`
   - `REQUIRE_CODE_OWNER_REVIEWS=false`
   - `REQUIRE_LAST_PUSH_APPROVAL=true`
3. `enterprise`
   - `REQUIRED_APPROVING_REVIEW_COUNT=2`
   - `REQUIRE_CODE_OWNER_REVIEWS=true`
   - `REQUIRE_LAST_PUSH_APPROVAL=true`

Checklist:

- [ ] Profile selected and documented.
- [ ] Repository variable `GOVERNANCE_PROFILE` set to target profile.
- [ ] Optional overrides reviewed:
  - `REQUIRED_APPROVING_REVIEW_COUNT`
  - `REQUIRE_CODE_OWNER_REVIEWS`
  - `REQUIRE_LAST_PUSH_APPROVAL`
  - `STRICT_SPRINT_DOC_REFERENCE` (`0`/`1` for governance sprint-doc enforcement strictness)

---

## 2. Apply and Verify Policy

Run policy commands for the selected profile:

```bash
GOVERNANCE_PROFILE=solo ./scripts/apply-repo-policy.sh 900Labs/900Invoice main
GOVERNANCE_PROFILE=solo ./scripts/verify-repo-policy.sh 900Labs/900Invoice main
```

Checklist:

- [ ] `apply-repo-policy.sh` completed successfully.
- [ ] `verify-repo-policy.sh` passed with `STRICT=1` (default).
- [ ] `Quality Gate` remains required in branch protection.
- [ ] Conversation resolution + linear history remain enabled.

---

## 3. Sprint Merge Hygiene

Checklist before squash merge:

- [ ] Sprint doc added in `docs/sprints/`.
- [ ] PR title/body clearly describe scope, validation, and risks.
- [ ] CI `Quality Gate` passed.
- [ ] Documentation updated with behavior/process changes.
- [ ] Governance-impacting sprint docs include `MAINTAINER_CHECKLIST_COMPLETION` block markers with checked items only.
- [ ] PR merged with **Squash and merge**.
- [ ] Merged PR URL and commit SHA shared with stakeholders.

Sprint-doc checklist completion block template:

```md
<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile/override expectations were reviewed for this sprint.
- [x] Policy verification and quality-gate evidence were captured in Validation.
- [x] Documentation and sprint records were updated for maintainer handoff.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
```

---

## 4. Governance Audit Triage

Checklist for governance incidents:

- [ ] Open workflow run and review failure logs.
- [ ] Download artifact: `governance-profile-assertion-<run_id>`.
- [ ] Confirm resolved profile and override values match expectation.
- [ ] Re-apply policy if drift is confirmed.
- [ ] Re-run governance audit and confirm pass.
- [ ] Update/close governance incident issue with remediation details.
- [ ] If external escalation is required, confirm `GOVERNANCE_INCIDENT_WEBHOOK_URL` secret is configured.
- [ ] If webhook receiver requires auth, confirm `GOVERNANCE_INCIDENT_WEBHOOK_TOKEN` secret is configured and valid.

---

## 5. Release Readiness

Checklist before creating a release tag:

- [ ] `main` is green.
- [ ] Governance verification passed.
- [ ] API docs parity verification passed.
- [ ] Release runbook checklist completed in `docs/RELEASE.md`.
- [ ] Changelog and sprint records updated.
