#!/usr/bin/env bash
set -euo pipefail

REPO="${1:-900Labs/900Invoice}"
BRANCH="${2:-main}"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT_DIR/scripts/governance-profile-env.sh"

if ! governance_profile_resolve; then
  exit 1
fi

echo "Applying repository merge policy for ${REPO}..."
gh api -X PATCH "repos/${REPO}" \
  -f allow_squash_merge=true \
  -f allow_merge_commit=false \
  -f allow_rebase_merge=false \
  -f delete_branch_on_merge=true >/dev/null
echo "Merge policy applied: squash-only + auto-delete merged branches."
echo "Resolved governance profile: ${GOVERNANCE_PROFILE} (reviews=${REQUIRED_APPROVING_REVIEW_COUNT}, code_owner=${REQUIRE_CODE_OWNER_REVIEWS}, last_push_approval=${REQUIRE_LAST_PUSH_APPROVAL})"

echo "Applying branch protection for ${REPO}:${BRANCH}..."
set +e
payload="$(jq -n \
  --argjson review_count "$REQUIRED_APPROVING_REVIEW_COUNT" \
  --argjson code_owner "$REQUIRE_CODE_OWNER_REVIEWS" \
  --argjson last_push "$REQUIRE_LAST_PUSH_APPROVAL" '{
  required_status_checks: {
    strict: true,
    contexts: ["Quality Gate"]
  },
  enforce_admins: true,
  required_pull_request_reviews: {
    dismiss_stale_reviews: true,
    require_code_owner_reviews: $code_owner,
    require_last_push_approval: $last_push,
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
