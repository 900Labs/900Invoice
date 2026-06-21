#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

REQUIRE_CLEAN_WORKTREE="${REQUIRE_CLEAN_WORKTREE:-1}"

log() {
  printf '\n==> %s\n' "$1"
}

fail() {
  echo "ERROR: $1" >&2
  exit 1
}

case "$REQUIRE_CLEAN_WORKTREE" in
  0|1) ;;
  *) fail "REQUIRE_CLEAN_WORKTREE must be 0 or 1" ;;
esac

command -v git >/dev/null 2>&1 || fail "git is required"

git_metadata_dirs() {
  local git_dir git_common_dir
  git_dir="$(git rev-parse --path-format=absolute --git-dir)" || fail "could not resolve git metadata path"
  git_common_dir="$(git rev-parse --path-format=absolute --git-common-dir)" || fail "could not resolve common git metadata path"

  printf '%s\n%s\n' "$git_dir" "$git_common_dir" | sort -u
}

check_macos_dataless_files() {
  if [[ "$(uname -s)" != "Darwin" ]]; then
    log "Skipping macOS dataless/offloaded-file check on $(uname -s)"
    return 0
  fi

  log "Checking for macOS dataless/offloaded git metadata"

  local tmp_git_dataless tmp_git_dirs tmp_tracked tmp_worktree_dataless tmp_missing
  tmp_git_dataless="$(mktemp)"
  tmp_git_dirs="$(mktemp)"
  tmp_tracked="$(mktemp)"
  tmp_worktree_dataless="$(mktemp)"
  tmp_missing="$(mktemp)"

  cleanup_dataless_check() {
    rm -f "$tmp_git_dataless" "$tmp_git_dirs" "$tmp_tracked" "$tmp_worktree_dataless" "$tmp_missing"
  }
  trap cleanup_dataless_check RETURN

  git_metadata_dirs > "$tmp_git_dirs"

  while IFS= read -r git_dir; do
    [[ -e "$git_dir" ]] || fail "git metadata path does not exist: $git_dir"
    find "$git_dir" -type f -flags +dataless -print
  done < "$tmp_git_dirs" | sort > "$tmp_git_dataless"

  if [[ -s "$tmp_git_dataless" ]]; then
    echo "ERROR: git metadata contains macOS dataless/offloaded files:" >&2
    sed 's/^/  - /' "$tmp_git_dataless" >&2
    echo >&2
    echo "Rehydrate the repository or clone it into a non-offloaded local directory before building or tagging." >&2
    echo "If iCloud Desktop/Documents sync uses Optimize Mac Storage, keep this checkout downloaded or place it outside that synced area." >&2
    exit 1
  fi

  log "Checking for missing or offloaded tracked files"
  git ls-files -z > "$tmp_tracked"

  while IFS= read -r -d '' path; do
    if [[ ! -e "$path" ]]; then
      printf '%s\n' "$path" >> "$tmp_missing"
      continue
    fi

    if [[ "$(stat -f '%Sf' "$path" 2>/dev/null || true)" == *dataless* ]]; then
      printf '%s\n' "$path" >> "$tmp_worktree_dataless"
    fi
  done < "$tmp_tracked"

  if [[ -s "$tmp_missing" ]]; then
    echo "ERROR: tracked files are missing from the working tree:" >&2
    sed 's/^/  - /' "$tmp_missing" >&2
    exit 1
  fi

  if [[ -s "$tmp_worktree_dataless" ]]; then
    echo "ERROR: tracked files are macOS dataless/offloaded:" >&2
    sed 's/^/  - /' "$tmp_worktree_dataless" >&2
    echo >&2
    echo "Rehydrate these files before running release checks or packaging builds." >&2
    echo "A fresh clone in a non-offloaded local directory is the most reliable recovery." >&2
    exit 1
  fi
}

check_git_health() {
  log "Checking git branch and object connectivity"
  git branch --show-current
  git rev-parse HEAD >/dev/null
  git fsck --connectivity-only
}

check_worktree_clean() {
  if [[ "$REQUIRE_CLEAN_WORKTREE" != "1" ]]; then
    log "Skipping clean worktree requirement (REQUIRE_CLEAN_WORKTREE=0)"
    return 0
  fi

  log "Checking tracked worktree cleanliness"

  local status_output
  status_output="$(git status --porcelain --untracked-files=no)"

  if [[ -n "$status_output" ]]; then
    echo "ERROR: tracked working tree is not clean:" >&2
    printf '%s\n' "$status_output" >&2
    exit 1
  fi
}

check_macos_dataless_files
check_git_health
check_worktree_clean

log "Local release preflight passed"
