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

jq -e '
  type == "object" and
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
  (.reason | type == "string" and length > 0)
' "${TRACE_PATH}" >/dev/null

jq -e '
  (keys | sort) == [
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
  ]
' "${TRACE_PATH}" >/dev/null

echo "Governance trace JSON schema validation passed: ${TRACE_PATH}"
