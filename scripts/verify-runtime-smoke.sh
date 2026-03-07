#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${ROOT_DIR}"

SMOKE_PROFILE="${SMOKE_PROFILE:-baseline}"
INSTALL_NODE_DEPS="${INSTALL_NODE_DEPS:-0}"
LEGACY_HARDWARE="${LEGACY_HARDWARE:-0}"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/900invoice-target-smoke}"
export CARGO_TARGET_DIR
SMOKE_TMPDIR="${SMOKE_TMPDIR:-/tmp/900invoice-smoke-tmp}"
mkdir -p "${SMOKE_TMPDIR}"
export TMPDIR="${SMOKE_TMPDIR}"

log() {
  printf '\n==> %s\n' "$1"
}

run() {
  log "$*"
  "$@"
}

case "${SMOKE_PROFILE}" in
  baseline|full) ;;
  *)
    echo "ERROR: SMOKE_PROFILE must be one of: baseline, full" >&2
    exit 1
    ;;
esac

if [[ "${INSTALL_NODE_DEPS}" != "0" && "${INSTALL_NODE_DEPS}" != "1" ]]; then
  echo "ERROR: INSTALL_NODE_DEPS must be 0 or 1" >&2
  exit 1
fi

if [[ "${LEGACY_HARDWARE}" != "0" && "${LEGACY_HARDWARE}" != "1" ]]; then
  echo "ERROR: LEGACY_HARDWARE must be 0 or 1" >&2
  exit 1
fi

if [[ "${LEGACY_HARDWARE}" == "1" ]]; then
  export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
  export CARGO_INCREMENTAL="0"
  export NPM_CONFIG_FUND="false"
  export NPM_CONFIG_AUDIT="false"
  export NPM_CONFIG_PROGRESS="false"
  log "Legacy hardware mode enabled (CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS})"
fi

run ./scripts/verify-api-doc-commands.sh

if [[ "${INSTALL_NODE_DEPS}" == "1" ]]; then
  run npm ci
elif [[ ! -d node_modules ]]; then
  echo "ERROR: node_modules missing; rerun with INSTALL_NODE_DEPS=1" >&2
  exit 1
fi

run npm run build
run cargo check --manifest-path src-tauri/Cargo.toml
run cargo test --manifest-path src-tauri/Cargo.toml

if [[ "${SMOKE_PROFILE}" == "full" ]]; then
  run cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
else
  log "Skipping cargo clippy in baseline profile"
fi

log "Runtime smoke verification passed (profile=${SMOKE_PROFILE}, legacy=${LEGACY_HARDWARE}, target_dir=${CARGO_TARGET_DIR}, tmpdir=${TMPDIR})"
