#!/usr/bin/env bash
set -euo pipefail

BASE_REF="${1:-origin/main}"
HEAD_REF="${2:-HEAD}"
REPORT_PATH="${REPORT_PATH:-}"

write_report_line() {
  [[ -z "$REPORT_PATH" ]] && return 0
  printf '%s\n' "$1" >> "$REPORT_PATH"
}

write_report_list() {
  local label="$1"
  shift
  write_report_line "${label}:"
  if [[ "$#" -eq 0 ]]; then
    write_report_line "  - (none)"
    return 0
  fi
  local item
  for item in "$@"; do
    write_report_line "  - ${item}"
  done
}

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
  : > "$REPORT_PATH"
fi

write_report_line "timestamp_utc: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
write_report_line "base_ref: ${BASE_REF}"
write_report_line "head_ref: ${HEAD_REF}"

changed_files="$(git diff --name-only "${BASE_REF}...${HEAD_REF}")"

if [[ -z "$changed_files" ]]; then
  write_report_line "result: pass"
  write_report_line "reason: no changed files in diff range"
  echo "No changed files detected between ${BASE_REF} and ${HEAD_REF}."
  exit 0
fi

changed_list=()
while IFS= read -r changed_file; do
  [[ -z "$changed_file" ]] && continue
  changed_list+=("$changed_file")
done <<< "$changed_files"
write_report_list "changed_files" "${changed_list[@]}"

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

governance_files=()
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  if is_governance_file "$file"; then
    governance_files+=("$file")
  fi
done <<< "$changed_files"

write_report_list "governance_changed_files" "${governance_files[@]}"

if [[ "${#governance_files[@]}" -eq 0 ]]; then
  write_report_line "result: pass"
  write_report_line "reason: no governance-impacting files changed"
  echo "Governance checklist enforcement skipped: no governance-impacting files changed."
  exit 0
fi

sprint_docs=()
while IFS= read -r sprint_doc; do
  [[ -z "$sprint_doc" ]] && continue
  sprint_docs+=("$sprint_doc")
done < <(echo "$changed_files" | awk '/^docs\/sprints\/sprint-[0-9]{3}.*\.md$/ {print}')

if [[ "${#sprint_docs[@]}" -eq 0 ]]; then
  write_report_list "evaluated_sprint_docs" "${sprint_docs[@]}"
  write_report_line "result: fail"
  write_report_line "reason: governance-impacting changes require sprint docs in docs/sprints/"
  echo "ERROR: Governance-impacting PRs must update a sprint doc in docs/sprints/." >&2
  exit 1
fi

write_report_list "evaluated_sprint_docs" "${sprint_docs[@]}"

has_checklist_ref=0
checklist_ref_docs=()
for sprint_doc in "${sprint_docs[@]}"; do
  if grep -Fq "docs/MAINTAINER_CHECKLIST.md" "$sprint_doc"; then
    has_checklist_ref=1
    checklist_ref_docs+=("$sprint_doc")
  fi
done

write_report_list "sprint_docs_with_checklist_reference" "${checklist_ref_docs[@]}"

if [[ "$has_checklist_ref" -eq 0 ]]; then
  write_report_line "result: fail"
  write_report_line "reason: sprint docs missing docs/MAINTAINER_CHECKLIST.md reference"
  echo "ERROR: Governance-impacting sprint docs must reference docs/MAINTAINER_CHECKLIST.md." >&2
  echo "Checked files:" >&2
  for sprint_doc in "${sprint_docs[@]}"; do
    echo "  - ${sprint_doc}" >&2
  done
  exit 1
fi

write_report_line "result: pass"
write_report_line "reason: governance sprint checklist enforcement passed"
echo "Governance sprint checklist enforcement passed."
