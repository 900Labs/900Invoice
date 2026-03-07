# Runtime Smoke Runbook

This runbook defines release-readiness runtime smoke checks for 900Invoice.

Use it after feature work is complete and before public release or major milestone merges.

---

## Why This Exists

900Invoice is built for constrained environments, including older hardware and intermittent connectivity. Runtime smoke checks ensure that the app still builds and verifies correctly under practical low-resource conditions.

---

## Profiles

The smoke script supports two profiles:

1. `baseline`
   - API docs parity check
   - frontend production build (`npm run build`)
   - Rust compile check
   - Rust unit tests
   - skips clippy for faster turnaround
2. `full`
   - everything in `baseline`
   - adds `cargo clippy -- -D warnings`

---

## Script

Run from repository root:

```bash
./scripts/verify-runtime-smoke.sh
```

Environment controls:

1. `SMOKE_PROFILE=baseline|full` (default `baseline`)
2. `INSTALL_NODE_DEPS=0|1` (default `0`; set `1` to run `npm ci`)
3. `LEGACY_HARDWARE=0|1` (default `0`)
4. `CARGO_TARGET_DIR=/tmp/900invoice-target-smoke` (default)
5. `SMOKE_TMPDIR=/tmp/900invoice-smoke-tmp` (default, exported as `TMPDIR`)

---

## Recommended Commands

Fast pre-PR smoke:

```bash
INSTALL_NODE_DEPS=1 SMOKE_PROFILE=baseline ./scripts/verify-runtime-smoke.sh
```

Low-resource hardware smoke:

```bash
INSTALL_NODE_DEPS=1 SMOKE_PROFILE=baseline LEGACY_HARDWARE=1 ./scripts/verify-runtime-smoke.sh
```

Release-grade smoke:

```bash
INSTALL_NODE_DEPS=1 SMOKE_PROFILE=full ./scripts/verify-runtime-smoke.sh
```

---

## Legacy Hardware Mode

When `LEGACY_HARDWARE=1`, the script applies conservative defaults:

1. `CARGO_BUILD_JOBS=1`
2. `CARGO_INCREMENTAL=0`
3. `NPM_CONFIG_FUND=false`
4. `NPM_CONFIG_AUDIT=false`
5. `NPM_CONFIG_PROGRESS=false`

These settings reduce peak memory/CPU pressure and avoid unnecessary network/audit overhead.

---

## Failure Handling

1. If `node_modules` is missing and `INSTALL_NODE_DEPS=0`, rerun with `INSTALL_NODE_DEPS=1`.
2. If `npm run build` fails after dependency install, clear local modules and reinstall:
   - `rm -rf node_modules`
   - `npm ci`
3. If Rust lock contention appears, keep `CARGO_TARGET_DIR` fixed for the whole run and avoid parallel cargo jobs.
4. If your machine is low on disk, clear stale temporary build targets before rerunning.

---

## Evidence for PRs

Include smoke results in PR validation, for example:

```text
Validation
- INSTALL_NODE_DEPS=1 SMOKE_PROFILE=baseline LEGACY_HARDWARE=1 ./scripts/verify-runtime-smoke.sh: passed
- INSTALL_NODE_DEPS=1 SMOKE_PROFILE=full ./scripts/verify-runtime-smoke.sh: passed
```
