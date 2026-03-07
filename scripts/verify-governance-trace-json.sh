#!/usr/bin/env bash
set -euo pipefail

TRACE_PATH="${1:-}"
SCHEMA_PATH="${2:-docs/schemas/governance-diff-trace.schema.json}"

if [[ -z "${TRACE_PATH}" ]]; then
  echo "Usage: $0 <trace-json-path> [schema-path]" >&2
  exit 1
fi

if [[ ! -f "${TRACE_PATH}" ]]; then
  echo "ERROR: Trace JSON not found: ${TRACE_PATH}" >&2
  exit 1
fi

if [[ ! -f "${SCHEMA_PATH}" ]]; then
  echo "ERROR: Schema file not found: ${SCHEMA_PATH}" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "ERROR: jq is required for governance trace validation." >&2
  exit 1
fi

jq -e . "${SCHEMA_PATH}" >/dev/null
jq -e . "${TRACE_PATH}" >/dev/null

trace_schema_version="$(jq -r '.schema_version // "1.0.0"' "${TRACE_PATH}")"
case "${trace_schema_version}" in
  1.0.0|1.1.0) ;;
  *)
    echo "ERROR: Unsupported governance trace schema_version: ${trace_schema_version}" >&2
    exit 1
    ;;
esac

legacy_keys='[
  "base_ref",
  "changed_files",
  "evaluated_sprint_docs",
  "governance_changed_files",
  "head_ref",
  "reason",
  "result",
  "sprint_docs_incomplete_checklist_completion_block",
  "sprint_docs_missing_checklist_completion_block",
  "sprint_docs_missing_checklist_reference",
  "sprint_docs_with_checklist_completion_block",
  "sprint_docs_with_checklist_reference",
  "strict_sprint_doc_reference",
  "timestamp_utc"
]'
legacy_keys_with_schema='[
  "base_ref",
  "changed_files",
  "evaluated_sprint_docs",
  "governance_changed_files",
  "head_ref",
  "reason",
  "result",
  "schema_version",
  "sprint_docs_incomplete_checklist_completion_block",
  "sprint_docs_missing_checklist_completion_block",
  "sprint_docs_missing_checklist_reference",
  "sprint_docs_with_checklist_completion_block",
  "sprint_docs_with_checklist_reference",
  "strict_sprint_doc_reference",
  "timestamp_utc"
]'
v11_keys='[
  "base_ref",
  "changed_files",
  "evaluated_sprint_docs",
  "governance_changed_files",
  "head_ref",
  "reason",
  "result",
  "schema_version",
  "sprint_docs_incomplete_checklist_completion_block",
  "sprint_docs_malformed_checklist_completion_block",
  "sprint_docs_missing_checklist_completion_block",
  "sprint_docs_missing_checklist_reference",
  "sprint_docs_with_checklist_completion_block",
  "sprint_docs_with_checklist_reference",
  "strict_sprint_doc_reference",
  "timestamp_utc"
]'

if [[ "${trace_schema_version}" == "1.0.0" ]]; then
  jq -e --argjson legacy_keys "${legacy_keys}" --argjson legacy_keys_with_schema "${legacy_keys_with_schema}" '
    type == "object" and
    ((has("schema_version") | not) or .schema_version == "1.0.0") and
    (.timestamp_utc | type == "string" and length > 0) and
    (.base_ref | type == "string" and length > 0) and
    (.head_ref | type == "string" and length > 0) and
    (.strict_sprint_doc_reference | type == "boolean") and
    (.changed_files | type == "array" and all(.[]?; type == "string")) and
    (.governance_changed_files | type == "array" and all(.[]?; type == "string")) and
    (.evaluated_sprint_docs | type == "array" and all(.[]?; type == "string")) and
    (.sprint_docs_with_checklist_reference | type == "array" and all(.[]?; type == "string")) and
    (.sprint_docs_missing_checklist_reference | type == "array" and all(.[]?; type == "string")) and
    (.sprint_docs_with_checklist_completion_block | type == "array" and all(.[]?; type == "string")) and
    (.sprint_docs_missing_checklist_completion_block | type == "array" and all(.[]?; type == "string")) and
    (.sprint_docs_incomplete_checklist_completion_block | type == "array" and all(.[]?; type == "string")) and
    (.result | type == "string" and (. == "pass" or . == "fail")) and
    (.reason | type == "string" and length > 0) and
    (
      (keys | sort) == $legacy_keys or
      (keys | sort) == $legacy_keys_with_schema
    )
  ' "${TRACE_PATH}" >/dev/null
else
  jq -e --argjson v11_keys "${v11_keys}" '
    type == "object" and
    (.schema_version == "1.1.0") and
    (.timestamp_utc | type == "string" and length > 0) and
    (.base_ref | type == "string" and length > 0) and
    (.head_ref | type == "string" and length > 0) and
    (.strict_sprint_doc_reference | type == "boolean") and
    (.changed_files | type == "array" and all(.[]?; type == "string")) and
    (.governance_changed_files | type == "array" and all(.[]?; type == "string")) and
    (.evaluated_sprint_docs | type == "array" and all(.[]?; type == "string")) and
    (.sprint_docs_with_checklist_reference | type == "array" and all(.[]?; type == "string")) and
    (.sprint_docs_missing_checklist_reference | type == "array" and all(.[]?; type == "string")) and
    (.sprint_docs_with_checklist_completion_block | type == "array" and all(.[]?; type == "string")) and
    (.sprint_docs_missing_checklist_completion_block | type == "array" and all(.[]?; type == "string")) and
    (.sprint_docs_incomplete_checklist_completion_block | type == "array" and all(.[]?; type == "string")) and
    (.sprint_docs_malformed_checklist_completion_block | type == "array" and all(.[]?; type == "string")) and
    (.result | type == "string" and (. == "pass" or . == "fail")) and
    (.reason | type == "string" and length > 0) and
    ((keys | sort) == $v11_keys)
  ' "${TRACE_PATH}" >/dev/null
fi

echo "Governance trace JSON schema validation passed: ${TRACE_PATH} (schema_version=${trace_schema_version})"
