#!/usr/bin/env bash
set -euo pipefail

BASE_REF="${1:-origin/main}"
HEAD_REF="${2:-HEAD}"

changed_files="$(git diff --name-only "${BASE_REF}...${HEAD_REF}")"

if [[ -z "$changed_files" ]]; then
  echo "No changed files detected between ${BASE_REF} and ${HEAD_REF}."
  exit 0
fi

is_governance_file() {
  local path="$1"
  case "$path" in
    .github/pull_request_template.md) return 0 ;;
    .github/workflows/governance-audit.yml) return 0 ;;
    .github/workflows/release.yml) return 0 ;;
    scripts/apply-repo-policy.sh) return 0 ;;
    scripts/verify-repo-policy.sh) return 0 ;;
    scripts/governance-profile-env.sh) return 0 ;;
    docs/BRANCH_PROTECTION.md) return 0 ;;
    docs/GOVERNANCE_AUDIT.md) return 0 ;;
    docs/MAINTAINER_CHECKLIST.md) return 0 ;;
    docs/RELEASE.md) return 0 ;;
    docs/SPRINT_PROCESS.md) return 0 ;;
    *) return 1 ;;
  esac
}

governance_touched=0
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  if is_governance_file "$file"; then
    governance_touched=1
    break
  fi
done <<< "$changed_files"

if [[ "$governance_touched" -eq 0 ]]; then
  echo "Governance checklist enforcement skipped: no governance-impacting files changed."
  exit 0
fi

sprint_docs=()
while IFS= read -r sprint_doc; do
  [[ -z "$sprint_doc" ]] && continue
  sprint_docs+=("$sprint_doc")
done < <(echo "$changed_files" | awk '/^docs\/sprints\/sprint-[0-9]{3}.*\.md$/ {print}')

if [[ "${#sprint_docs[@]}" -eq 0 ]]; then
  echo "ERROR: Governance-impacting PRs must update a sprint doc in docs/sprints/." >&2
  exit 1
fi

has_checklist_ref=0
for sprint_doc in "${sprint_docs[@]}"; do
  if grep -Fq "docs/MAINTAINER_CHECKLIST.md" "$sprint_doc"; then
    has_checklist_ref=1
    break
  fi
done

if [[ "$has_checklist_ref" -eq 0 ]]; then
  echo "ERROR: Governance-impacting sprint docs must reference docs/MAINTAINER_CHECKLIST.md." >&2
  echo "Checked files:" >&2
  for sprint_doc in "${sprint_docs[@]}"; do
    echo "  - ${sprint_doc}" >&2
  done
  exit 1
fi

echo "Governance sprint checklist enforcement passed."
