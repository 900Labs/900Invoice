#!/usr/bin/env bash
set -euo pipefail

REPO="${1:-900Labs/900Invoice}"
BRANCH="${2:-main}"

echo "Applying repository merge policy for ${REPO}..."
gh api -X PATCH "repos/${REPO}" \
  -f allow_squash_merge=true \
  -f allow_merge_commit=false \
  -f allow_rebase_merge=false \
  -f delete_branch_on_merge=true >/dev/null
echo "Merge policy applied: squash-only + auto-delete merged branches."

echo "Applying branch protection for ${REPO}:${BRANCH}..."
set +e
resp="$(
  gh api -X PUT "repos/${REPO}/branches/${BRANCH}/protection" \
    -H "Accept: application/vnd.github+json" \
    -f required_status_checks.strict=true \
    -f required_status_checks.contexts[]="Quality Gate" \
    -f enforce_admins=true \
    -f required_pull_request_reviews.dismiss_stale_reviews=true \
    -f required_pull_request_reviews.require_code_owner_reviews=false \
    -f required_pull_request_reviews.required_approving_review_count=1 \
    -f required_conversation_resolution=true \
    -f restrictions= \
    -f allow_force_pushes=false \
    -f allow_deletions=false \
    -f block_creations=false \
    -f required_linear_history=true 2>&1
)"
status=$?
set -e

if [[ $status -eq 0 ]]; then
  echo "Branch protection applied for ${BRANCH}."
  exit 0
fi

if echo "$resp" | rg -q "Upgrade to GitHub Pro or make this repository public"; then
  cat <<EOF
NOTICE: Branch protection could not be enabled yet.
Reason: GitHub returned a plan/visibility restriction for private repositories.

Next step:
1) Make the repository public (or upgrade plan support).
2) Re-run:
   ./scripts/apply-repo-policy.sh ${REPO} ${BRANCH}
EOF
  exit 0
fi

echo "ERROR: Failed to apply branch protection." >&2
echo "$resp" >&2
exit 1
