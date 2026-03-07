#!/usr/bin/env bash
set -euo pipefail

BASE_REF="${1:-origin/main}"
HEAD_REF="${2:-HEAD}"
REPORT_PATH="${REPORT_PATH:-}"
REPORT_JSON_PATH="${REPORT_JSON_PATH:-}"
STRICT_SPRINT_DOC_REFERENCE="${STRICT_SPRINT_DOC_REFERENCE:-0}"
TRACE_SCHEMA_VERSION="1.1.0"

changed_list=()
governance_files=()
sprint_docs=()
checklist_ref_docs=()
missing_checklist_ref_docs=()
checklist_completion_block_docs=()
missing_checklist_completion_block_docs=()
incomplete_checklist_completion_block_docs=()
malformed_checklist_completion_block_docs=()
RESULT="unknown"
REASON="script execution did not complete"
TIMESTAMP_UTC="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

case "$STRICT_SPRINT_DOC_REFERENCE" in
  1|true|TRUE|yes|YES) STRICT_SPRINT_DOC_REFERENCE_BOOL=true ;;
  0|false|FALSE|no|NO|"") STRICT_SPRINT_DOC_REFERENCE_BOOL=false ;;
  *)
    echo "ERROR: STRICT_SPRINT_DOC_REFERENCE must be 0/1/true/false." >&2
    exit 1
    ;;
esac

array_to_json() {
  if [[ "$#" -eq 0 ]]; then
    printf '[]'
    return 0
  fi
  printf '%s\n' "$@" | jq -R . | jq -s .
}

emit_json_report() {
  set +u
  [[ -z "$REPORT_JSON_PATH" ]] && return 0
  mkdir -p "$(dirname "$REPORT_JSON_PATH")"

  if ! command -v jq >/dev/null 2>&1; then
    cat > "$REPORT_JSON_PATH" <<EOF
{"timestamp_utc":"$TIMESTAMP_UTC","base_ref":"$BASE_REF","head_ref":"$HEAD_REF","result":"$RESULT","reason":"$REASON","error":"jq not available"}
EOF
    return 0
  fi

  local changed_json governance_json sprint_docs_json checklist_json missing_json completion_json completion_missing_json completion_incomplete_json completion_malformed_json
  changed_json="$(array_to_json "${changed_list[@]}")"
  governance_json="$(array_to_json "${governance_files[@]}")"
  sprint_docs_json="$(array_to_json "${sprint_docs[@]}")"
  checklist_json="$(array_to_json "${checklist_ref_docs[@]}")"
  missing_json="$(array_to_json "${missing_checklist_ref_docs[@]}")"
  completion_json="$(array_to_json "${checklist_completion_block_docs[@]}")"
  completion_missing_json="$(array_to_json "${missing_checklist_completion_block_docs[@]}")"
  completion_incomplete_json="$(array_to_json "${incomplete_checklist_completion_block_docs[@]}")"
  completion_malformed_json="$(array_to_json "${malformed_checklist_completion_block_docs[@]}")"

  jq -n \
    --arg schema_version "$TRACE_SCHEMA_VERSION" \
    --arg timestamp_utc "$TIMESTAMP_UTC" \
    --arg base_ref "$BASE_REF" \
    --arg head_ref "$HEAD_REF" \
    --arg result "$RESULT" \
    --arg reason "$REASON" \
    --argjson strict_sprint_doc_reference "$STRICT_SPRINT_DOC_REFERENCE_BOOL" \
    --argjson changed_files "$changed_json" \
    --argjson governance_changed_files "$governance_json" \
    --argjson evaluated_sprint_docs "$sprint_docs_json" \
    --argjson sprint_docs_with_checklist_reference "$checklist_json" \
    --argjson sprint_docs_missing_checklist_reference "$missing_json" \
    --argjson sprint_docs_with_checklist_completion_block "$completion_json" \
    --argjson sprint_docs_missing_checklist_completion_block "$completion_missing_json" \
    --argjson sprint_docs_incomplete_checklist_completion_block "$completion_incomplete_json" \
    --argjson sprint_docs_malformed_checklist_completion_block "$completion_malformed_json" \
    '{
      schema_version: $schema_version,
      timestamp_utc: $timestamp_utc,
      base_ref: $base_ref,
      head_ref: $head_ref,
      strict_sprint_doc_reference: $strict_sprint_doc_reference,
      changed_files: $changed_files,
      governance_changed_files: $governance_changed_files,
      evaluated_sprint_docs: $evaluated_sprint_docs,
      sprint_docs_with_checklist_reference: $sprint_docs_with_checklist_reference,
      sprint_docs_missing_checklist_reference: $sprint_docs_missing_checklist_reference,
      sprint_docs_with_checklist_completion_block: $sprint_docs_with_checklist_completion_block,
      sprint_docs_missing_checklist_completion_block: $sprint_docs_missing_checklist_completion_block,
      sprint_docs_incomplete_checklist_completion_block: $sprint_docs_incomplete_checklist_completion_block,
      sprint_docs_malformed_checklist_completion_block: $sprint_docs_malformed_checklist_completion_block,
      result: $result,
      reason: $reason
    }' > "$REPORT_JSON_PATH"
}

trap emit_json_report EXIT

write_report_line() {
  [[ -z "$REPORT_PATH" ]] && return 0
  printf '%s\n' "$1" >> "$REPORT_PATH"
}

write_report_list() {
  local label="$1"
  shift
  write_report_line "${label}:"
  local filtered=()
  local item
  for item in "$@"; do
    [[ -z "$item" ]] && continue
    filtered+=("$item")
  done
  if [[ "${#filtered[@]}" -eq 0 ]]; then
    write_report_line "  - (none)"
    return 0
  fi
  for item in "${filtered[@]}"; do
    write_report_line "  - ${item}"
  done
}

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
  : > "$REPORT_PATH"
fi

write_report_line "timestamp_utc: ${TIMESTAMP_UTC}"
write_report_line "base_ref: ${BASE_REF}"
write_report_line "head_ref: ${HEAD_REF}"
write_report_line "strict_sprint_doc_reference: ${STRICT_SPRINT_DOC_REFERENCE_BOOL}"
write_report_line "schema_version: ${TRACE_SCHEMA_VERSION}"

changed_files="$(git diff --name-only "${BASE_REF}...${HEAD_REF}")"

if [[ -z "$changed_files" ]]; then
  RESULT="pass"
  REASON="no changed files in diff range"
  write_report_line "result: pass"
  write_report_line "reason: no changed files in diff range"
  echo "No changed files detected between ${BASE_REF} and ${HEAD_REF}."
  exit 0
fi

while IFS= read -r changed_file; do
  [[ -z "$changed_file" ]] && continue
  changed_list+=("$changed_file")
done <<< "$changed_files"
write_report_list "changed_files" "${changed_list[@]-}"

is_governance_file() {
  local path="$1"
  case "$path" in
    .github/pull_request_template.md) return 0 ;;
    .github/workflows/ci.yml) return 0 ;;
    .github/workflows/governance-artifact-inventory.yml) return 0 ;;
    .github/workflows/governance-audit.yml) return 0 ;;
    .github/workflows/release.yml) return 0 ;;
    scripts/apply-repo-policy.sh) return 0 ;;
    scripts/verify-governance-sprint-checklist.sh) return 0 ;;
    scripts/verify-governance-trace-json.sh) return 0 ;;
    scripts/verify-repo-policy.sh) return 0 ;;
    scripts/governance-profile-env.sh) return 0 ;;
    docs/BRANCH_PROTECTION.md) return 0 ;;
    docs/GOVERNANCE_AUDIT.md) return 0 ;;
    docs/MAINTAINER_CHECKLIST.md) return 0 ;;
    docs/QUALITY_GATE.md) return 0 ;;
    docs/RELEASE.md) return 0 ;;
    docs/schemas/governance-diff-trace.schema.json) return 0 ;;
    docs/SPRINT_PROCESS.md) return 0 ;;
    *) return 1 ;;
  esac
}

while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  if is_governance_file "$file"; then
    governance_files+=("$file")
  fi
done <<< "$changed_files"

write_report_list "governance_changed_files" "${governance_files[@]-}"

if [[ "${#governance_files[@]}" -eq 0 ]]; then
  RESULT="pass"
  REASON="no governance-impacting files changed"
  write_report_line "result: pass"
  write_report_line "reason: no governance-impacting files changed"
  echo "Governance checklist enforcement skipped: no governance-impacting files changed."
  exit 0
fi

while IFS= read -r sprint_doc; do
  [[ -z "$sprint_doc" ]] && continue
  sprint_docs+=("$sprint_doc")
done < <(echo "$changed_files" | awk '/^docs\/sprints\/sprint-[0-9]{3}.*\.md$/ {print}')

if [[ "${#sprint_docs[@]}" -eq 0 ]]; then
  RESULT="fail"
  REASON="governance-impacting changes require sprint docs in docs/sprints/"
  write_report_list "evaluated_sprint_docs" "${sprint_docs[@]-}"
  write_report_line "result: fail"
  write_report_line "reason: governance-impacting changes require sprint docs in docs/sprints/"
  echo "ERROR: Governance-impacting PRs must update a sprint doc in docs/sprints/." >&2
  exit 1
fi

write_report_list "evaluated_sprint_docs" "${sprint_docs[@]-}"

has_checklist_ref=0
for sprint_doc in "${sprint_docs[@]}"; do
  if grep -Fq "docs/MAINTAINER_CHECKLIST.md" "$sprint_doc"; then
    has_checklist_ref=1
    checklist_ref_docs+=("$sprint_doc")
  else
    missing_checklist_ref_docs+=("$sprint_doc")
  fi
done

write_report_list "sprint_docs_with_checklist_reference" "${checklist_ref_docs[@]-}"
write_report_list "sprint_docs_missing_checklist_reference" "${missing_checklist_ref_docs[@]-}"

checklist_completion_block_status() {
  local file="$1"
  awk '
    /<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->/ {
      begin_count++
      if (in_block == 1) nested = 1
      in_block = 1
      next
    }
    /<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->/ {
      end_count++
      if (in_block == 0) unbalanced = 1
      if (in_block == 1) in_block = 0
      next
    }
    in_block == 1 {
      if ($0 ~ /^- \[[xX]\] /) checked++
      if ($0 ~ /^- \[[[:space:]]\] /) unchecked++
    }
    END {
      if (begin_count == 0 && end_count == 0) {
        print "missing"
        exit
      }
      if (begin_count != 1 || end_count != 1 || nested == 1 || unbalanced == 1 || in_block == 1) {
        print "malformed"
        exit
      }
      if (checked < 3 || unchecked > 0) {
        print "incomplete"
        exit
      }
      print "valid"
    }
  ' "$file"
}

for sprint_doc in "${sprint_docs[@]}"; do
  completion_status="$(checklist_completion_block_status "$sprint_doc")"
  case "$completion_status" in
    valid) checklist_completion_block_docs+=("$sprint_doc") ;;
    missing) missing_checklist_completion_block_docs+=("$sprint_doc") ;;
    malformed) malformed_checklist_completion_block_docs+=("$sprint_doc") ;;
    incomplete) incomplete_checklist_completion_block_docs+=("$sprint_doc") ;;
    *)
      incomplete_checklist_completion_block_docs+=("$sprint_doc")
      ;;
  esac
done

write_report_list "sprint_docs_with_checklist_completion_block" "${checklist_completion_block_docs[@]-}"
write_report_list "sprint_docs_missing_checklist_completion_block" "${missing_checklist_completion_block_docs[@]-}"
write_report_list "sprint_docs_malformed_checklist_completion_block" "${malformed_checklist_completion_block_docs[@]-}"
write_report_list "sprint_docs_incomplete_checklist_completion_block" "${incomplete_checklist_completion_block_docs[@]-}"

if [[ "$STRICT_SPRINT_DOC_REFERENCE_BOOL" == "true" && "${#missing_checklist_ref_docs[@]}" -gt 0 ]]; then
  RESULT="fail"
  REASON="strict mode requires checklist reference in all changed sprint docs"
  write_report_line "result: fail"
  write_report_line "reason: strict mode requires checklist reference in all changed sprint docs"
  echo "ERROR: STRICT_SPRINT_DOC_REFERENCE=true and not all changed sprint docs reference docs/MAINTAINER_CHECKLIST.md." >&2
  echo "Missing reference:" >&2
  for sprint_doc in "${missing_checklist_ref_docs[@]}"; do
    echo "  - ${sprint_doc}" >&2
  done
  exit 1
fi

if [[ "${#missing_checklist_completion_block_docs[@]}" -gt 0 ]]; then
  RESULT="fail"
  REASON="changed sprint docs must include checklist completion block markers"
  write_report_line "result: fail"
  write_report_line "reason: changed sprint docs must include checklist completion block markers"
  echo "ERROR: Changed sprint docs must include checklist completion block markers:" >&2
  echo "  <!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->" >&2
  echo "  - [x] ..." >&2
  echo "  - [x] ..." >&2
  echo "  - [x] ..." >&2
  echo "  <!-- MAINTAINER_CHECKLIST_COMPLETION:END -->" >&2
  echo "Missing block in:" >&2
  for sprint_doc in "${missing_checklist_completion_block_docs[@]}"; do
    echo "  - ${sprint_doc}" >&2
  done
  exit 1
fi

if [[ "${#malformed_checklist_completion_block_docs[@]}" -gt 0 ]]; then
  RESULT="fail"
  REASON="checklist completion block markers must not be duplicated, nested, or unbalanced"
  write_report_line "result: fail"
  write_report_line "reason: checklist completion block markers must not be duplicated, nested, or unbalanced"
  echo "ERROR: Checklist completion block markers are malformed (duplicate, nested, or unbalanced)." >&2
  echo "Malformed block in:" >&2
  for sprint_doc in "${malformed_checklist_completion_block_docs[@]}"; do
    echo "  - ${sprint_doc}" >&2
  done
  exit 1
fi

if [[ "${#incomplete_checklist_completion_block_docs[@]}" -gt 0 ]]; then
  RESULT="fail"
  REASON="checklist completion block must include at least three checked items and no unchecked items"
  write_report_line "result: fail"
  write_report_line "reason: checklist completion block must include at least three checked items and no unchecked items"
  echo "ERROR: Checklist completion block must include at least three '- [x]' items and no unchecked '- [ ]' items." >&2
  echo "Incomplete block in:" >&2
  for sprint_doc in "${incomplete_checklist_completion_block_docs[@]}"; do
    echo "  - ${sprint_doc}" >&2
  done
  exit 1
fi

if [[ "$has_checklist_ref" -eq 0 ]]; then
  RESULT="fail"
  REASON="sprint docs missing docs/MAINTAINER_CHECKLIST.md reference"
  write_report_line "result: fail"
  write_report_line "reason: sprint docs missing docs/MAINTAINER_CHECKLIST.md reference"
  echo "ERROR: Governance-impacting sprint docs must reference docs/MAINTAINER_CHECKLIST.md." >&2
  echo "Checked files:" >&2
  for sprint_doc in "${sprint_docs[@]}"; do
    echo "  - ${sprint_doc}" >&2
  done
  exit 1
fi

RESULT="pass"
REASON="governance sprint checklist enforcement passed"
write_report_line "result: pass"
write_report_line "reason: governance sprint checklist enforcement passed"
echo "Governance sprint checklist enforcement passed."
