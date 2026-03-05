#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LIB_RS="$ROOT_DIR/src-tauri/src/lib.rs"
API_MD="$ROOT_DIR/docs/API.md"

if [[ ! -f "$LIB_RS" ]]; then
  echo "ERROR: missing $LIB_RS" >&2
  exit 1
fi

if [[ ! -f "$API_MD" ]]; then
  echo "ERROR: missing $API_MD" >&2
  exit 1
fi

tmp_actual="$(mktemp)"
tmp_doc_raw="$(mktemp)"
tmp_doc="$(mktemp)"
tmp_missing="$(mktemp)"
tmp_extra="$(mktemp)"
tmp_dupes="$(mktemp)"
cleanup() {
  rm -f "$tmp_actual" "$tmp_doc_raw" "$tmp_doc" "$tmp_missing" "$tmp_extra" "$tmp_dupes"
}
trap cleanup EXIT

if command -v rg >/dev/null 2>&1; then
  rg -o "commands::[a-z_]+::[a-z_]+" "$LIB_RS" \
    | sed 's/.*:://' \
    | sort -u > "$tmp_actual"
else
  grep -oE "commands::[a-z_]+::[a-z_]+" "$LIB_RS" \
    | sed 's/.*:://' \
    | sort -u > "$tmp_actual"
fi

awk '
  /COMMAND_CATALOG_START/ { in_block = 1; next }
  /COMMAND_CATALOG_END/ { in_block = 0; next }
  in_block { print }
' "$API_MD" \
  | sed -E -n 's/^- `([a-z_]+)`$/\1/p' > "$tmp_doc_raw"

if [[ ! -s "$tmp_doc_raw" ]]; then
  echo "ERROR: docs/API.md command catalog block is empty or malformed." >&2
  exit 1
fi

sort "$tmp_doc_raw" > "$tmp_doc"

sort "$tmp_doc_raw" | uniq -d > "$tmp_dupes"
if [[ -s "$tmp_dupes" ]]; then
  echo "ERROR: duplicate commands found in docs/API.md catalog:" >&2
  sed 's/^/  - /' "$tmp_dupes" >&2
  exit 1
fi

comm -23 "$tmp_actual" "$tmp_doc" > "$tmp_missing"
comm -13 "$tmp_actual" "$tmp_doc" > "$tmp_extra"

if [[ -s "$tmp_missing" || -s "$tmp_extra" ]]; then
  echo "ERROR: docs/API.md command catalog is out of sync with src-tauri/src/lib.rs." >&2
  if [[ -s "$tmp_missing" ]]; then
    echo "Missing in docs/API.md:" >&2
    sed 's/^/  - /' "$tmp_missing" >&2
  fi
  if [[ -s "$tmp_extra" ]]; then
    echo "Extra in docs/API.md:" >&2
    sed 's/^/  - /' "$tmp_extra" >&2
  fi
  exit 1
fi

count="$(wc -l < "$tmp_actual" | tr -d ' ')"
echo "API command catalog is in sync (${count} commands)."
