#!/usr/bin/env bash
set -euo pipefail

REPO="${1:-900Labs/900Invoice}"
BRANCH="${2:-main}"
STRICT="${STRICT:-1}"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT_DIR/scripts/governance-profile-env.sh"

if ! governance_profile_resolve; then
  exit 1
fi

fail=0

echo "Verifying repository merge policy for ${REPO}..."
echo "Resolved governance profile: ${GOVERNANCE_PROFILE} (reviews=${REQUIRED_APPROVING_REVIEW_COUNT}, code_owner=${REQUIRE_CODE_OWNER_REVIEWS}, last_push_approval=${REQUIRE_LAST_PUSH_APPROVAL})"
repo_json="$(gh api "repos/${REPO}")"

allow_squash="$(echo "$repo_json" | jq -r '.allow_squash_merge')"
allow_merge_commit="$(echo "$repo_json" | jq -r '.allow_merge_commit')"
allow_rebase_merge="$(echo "$repo_json" | jq -r '.allow_rebase_merge')"
delete_branch_on_merge="$(echo "$repo_json" | jq -r '.delete_branch_on_merge')"

[[ "$allow_squash" == "true" ]] || { echo "FAIL: allow_squash_merge must be true"; fail=1; }
[[ "$allow_merge_commit" == "false" ]] || { echo "FAIL: allow_merge_commit must be false"; fail=1; }
[[ "$allow_rebase_merge" == "false" ]] || { echo "FAIL: allow_rebase_merge must be false"; fail=1; }
[[ "$delete_branch_on_merge" == "true" ]] || { echo "FAIL: delete_branch_on_merge must be true"; fail=1; }

echo "Verifying branch protection for ${REPO}:${BRANCH}..."
set +e
protection_json="$(gh api "repos/${REPO}/branches/${BRANCH}/protection" 2>&1)"
status=$?
set -e

if [[ $status -ne 0 ]]; then
  if echo "$protection_json" | grep -q "Upgrade to GitHub Pro or make this repository public"; then
    echo "NOTICE: branch protection unavailable due to private-repo plan/visibility restriction."
    if [[ "$STRICT" == "1" ]]; then
      echo "FAIL: STRICT=1 and branch protection is unavailable."
      fail=1
    fi
  else
    echo "FAIL: could not read branch protection:"
    echo "$protection_json"
    fail=1
  fi
else
  require_admins="$(echo "$protection_json" | jq -r '.enforce_admins.enabled')"
  strict_checks="$(echo "$protection_json" | jq -r '.required_status_checks.strict')"
  contexts="$(echo "$protection_json" | jq -r '.required_status_checks.contexts[]?')"
  reviews="$(echo "$protection_json" | jq -r '.required_pull_request_reviews.required_approving_review_count')"
  code_owner_reviews="$(echo "$protection_json" | jq -r '.required_pull_request_reviews.require_code_owner_reviews')"
  last_push_approval="$(echo "$protection_json" | jq -r '.required_pull_request_reviews.require_last_push_approval')"
  conv="$(echo "$protection_json" | jq -r '.required_conversation_resolution.enabled')"
  linear="$(echo "$protection_json" | jq -r '.required_linear_history.enabled')"

  [[ "$require_admins" == "true" ]] || { echo "FAIL: enforce_admins.enabled must be true"; fail=1; }
  [[ "$strict_checks" == "true" ]] || { echo "FAIL: required_status_checks.strict must be true"; fail=1; }
  echo "$contexts" | grep -Fxq "Quality Gate" || { echo "FAIL: required checks must include 'Quality Gate'"; fail=1; }
  [[ "$reviews" -eq "$REQUIRED_APPROVING_REVIEW_COUNT" ]] || {
    echo "FAIL: required approving reviews must be ${REQUIRED_APPROVING_REVIEW_COUNT}"
    fail=1
  }
  [[ "$code_owner_reviews" == "$REQUIRE_CODE_OWNER_REVIEWS" ]] || {
    echo "FAIL: require_code_owner_reviews must be ${REQUIRE_CODE_OWNER_REVIEWS}"
    fail=1
  }
  [[ "$last_push_approval" == "$REQUIRE_LAST_PUSH_APPROVAL" ]] || {
    echo "FAIL: require_last_push_approval must be ${REQUIRE_LAST_PUSH_APPROVAL}"
    fail=1
  }
  [[ "$conv" == "true" ]] || { echo "FAIL: required conversation resolution must be enabled"; fail=1; }
  [[ "$linear" == "true" ]] || { echo "FAIL: required linear history must be enabled"; fail=1; }
fi

if [[ $fail -ne 0 ]]; then
  exit 1
fi

echo "Repository policy verification passed."
