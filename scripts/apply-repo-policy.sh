#!/usr/bin/env bash
set -euo pipefail

REPO="${1:-900Labs/900Invoice}"
BRANCH="${2:-main}"
REQUIRED_APPROVING_REVIEW_COUNT="${REQUIRED_APPROVING_REVIEW_COUNT:-0}"

if [[ ! "$REQUIRED_APPROVING_REVIEW_COUNT" =~ ^[0-9]+$ ]]; then
  echo "ERROR: REQUIRED_APPROVING_REVIEW_COUNT must be a non-negative integer." >&2
  exit 1
fi

echo "Applying repository merge policy for ${REPO}..."
gh api -X PATCH "repos/${REPO}" \
  -f allow_squash_merge=true \
  -f allow_merge_commit=false \
  -f allow_rebase_merge=false \
  -f delete_branch_on_merge=true >/dev/null
echo "Merge policy applied: squash-only + auto-delete merged branches."

echo "Applying branch protection for ${REPO}:${BRANCH}..."
set +e
payload="$(jq -n --argjson review_count "$REQUIRED_APPROVING_REVIEW_COUNT" '{
  required_status_checks: {
    strict: true,
    contexts: ["Quality Gate"]
  },
  enforce_admins: true,
  required_pull_request_reviews: {
    dismiss_stale_reviews: true,
    require_code_owner_reviews: false,
    required_approving_review_count: $review_count
  },
  restrictions: null,
  required_conversation_resolution: true,
  allow_force_pushes: false,
  allow_deletions: false,
  block_creations: false,
  required_linear_history: true
}')"
resp="$(
  gh api -X PUT "repos/${REPO}/branches/${BRANCH}/protection" \
    -H "Accept: application/vnd.github+json" \
    --input - 2>&1 <<<"$payload"
)"
status=$?
set -e

if [[ $status -eq 0 ]]; then
  echo "Branch protection applied for ${BRANCH}."
  exit 0
fi

if echo "$resp" | grep -q "Upgrade to GitHub Pro or make this repository public"; then
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
