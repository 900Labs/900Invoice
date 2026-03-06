#!/usr/bin/env bash

governance_profile_resolve() {
  GOVERNANCE_PROFILE="${GOVERNANCE_PROFILE:-solo}"
  GOVERNANCE_PROFILE="$(echo "$GOVERNANCE_PROFILE" | tr '[:upper:]' '[:lower:]')"

  local default_reviews
  local default_code_owner
  local default_last_push

  case "$GOVERNANCE_PROFILE" in
    solo)
      default_reviews=0
      default_code_owner=false
      default_last_push=false
      ;;
    small-team)
      default_reviews=1
      default_code_owner=false
      default_last_push=true
      ;;
    enterprise)
      default_reviews=2
      default_code_owner=true
      default_last_push=true
      ;;
    *)
      echo "ERROR: GOVERNANCE_PROFILE must be one of: solo, small-team, enterprise." >&2
      return 1
      ;;
  esac

  REQUIRED_APPROVING_REVIEW_COUNT="${REQUIRED_APPROVING_REVIEW_COUNT:-$default_reviews}"
  REQUIRE_CODE_OWNER_REVIEWS="${REQUIRE_CODE_OWNER_REVIEWS:-$default_code_owner}"
  REQUIRE_LAST_PUSH_APPROVAL="${REQUIRE_LAST_PUSH_APPROVAL:-$default_last_push}"

  if [[ ! "$REQUIRED_APPROVING_REVIEW_COUNT" =~ ^[0-9]+$ ]]; then
    echo "ERROR: REQUIRED_APPROVING_REVIEW_COUNT must be a non-negative integer." >&2
    return 1
  fi

  case "$REQUIRE_CODE_OWNER_REVIEWS" in
    true|false) ;;
    *)
      echo "ERROR: REQUIRE_CODE_OWNER_REVIEWS must be true or false." >&2
      return 1
      ;;
  esac

  case "$REQUIRE_LAST_PUSH_APPROVAL" in
    true|false) ;;
    *)
      echo "ERROR: REQUIRE_LAST_PUSH_APPROVAL must be true or false." >&2
      return 1
      ;;
  esac
}
