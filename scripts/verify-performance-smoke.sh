#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

: "${PERF_SMOKE_CLIENTS:=500}"
: "${PERF_SMOKE_PRODUCTS:=300}"
: "${PERF_SMOKE_INVOICES:=1000}"
: "${PERF_SMOKE_LINES_PER_INVOICE:=3}"
: "${PERF_SMOKE_ENFORCE_BUDGETS:=1}"
: "${CARGO_TARGET_DIR:=/tmp/900invoice-perf-smoke-target}"

export PERF_SMOKE_CLIENTS
export PERF_SMOKE_PRODUCTS
export PERF_SMOKE_INVOICES
export PERF_SMOKE_LINES_PER_INVOICE
export PERF_SMOKE_ENFORCE_BUDGETS
export CARGO_TARGET_DIR

echo "Performance smoke profile:"
echo "  clients=${PERF_SMOKE_CLIENTS}"
echo "  products=${PERF_SMOKE_PRODUCTS}"
echo "  invoices=${PERF_SMOKE_INVOICES}"
echo "  lines_per_invoice=${PERF_SMOKE_LINES_PER_INVOICE}"
echo "  enforce_budgets=${PERF_SMOKE_ENFORCE_BUDGETS}"
echo "  cargo_target_dir=${CARGO_TARGET_DIR}"

cargo test \
  --manifest-path src-tauri/Cargo.toml \
  perf_smoke_large_dataset_hot_paths \
  -- --ignored --nocapture
