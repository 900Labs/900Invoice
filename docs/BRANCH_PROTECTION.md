# Branch Protection Policy

This document defines repository-level protection rules for `main`.

Current status (2026-03-05):

1. Repository merge policy is enforced.
2. Branch protection on `main` is active.

---

## Policy Targets

### Repository Merge Settings

Required:

1. `allow_squash_merge = true`
2. `allow_merge_commit = false`
3. `allow_rebase_merge = false`
4. `delete_branch_on_merge = true`

### Branch Protection (`main`)

Required when GitHub plan/visibility allows branch protection:

1. Require pull request before merge.
2. Require at least 1 approving review.
3. Dismiss stale approvals on new commits.
4. Require status checks to pass before merge.
5. Required status check context includes `Quality Gate`.
6. Require conversation resolution before merge.
7. Enforce branch protection for admins.
8. Require linear history.
9. Disallow force pushes and deletions.

---

## Apply Policy

```bash
./scripts/apply-repo-policy.sh 900Labs/900Invoice main
```

Notes:

1. The script always applies repository merge settings.
2. If branch protection is unavailable (for example private-repo plan restriction), the script prints a notice and exits successfully.
3. Re-run the script after making the repository public to enable branch protection.

---

## Verify Policy

```bash
./scripts/verify-repo-policy.sh 900Labs/900Invoice main
```

Strict mode:

```bash
STRICT=1 ./scripts/verify-repo-policy.sh 900Labs/900Invoice main
```

`STRICT=1` is the default. Set `STRICT=0` only for temporary diagnostics.

---

## Operational Guidance

1. Run `apply-repo-policy.sh` after repository creation or transfer.
2. Run `verify-repo-policy.sh` after any admin/settings changes.
3. Record policy changes in sprint docs and changelog.
