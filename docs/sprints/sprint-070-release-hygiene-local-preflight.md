# Sprint 070 - Release Hygiene And Local Preflight

Status: Completed
Date: 2026-06-21
Owner: 900 Labs

## Context

Sprint 069 completed the dependency and security cleanup, but the local checkout later showed macOS `compressed,dataless` placeholders in `.git` metadata and tracked source files. That caused small git reads such as `git status`, branch resolution, and ref reads to time out.

Before tagging or building platform artifacts, maintainers need a quick local preflight that catches this condition before release commands, dependency installs, or Tauri packaging work begin.

## Goals

1. Detect macOS dataless/offloaded repository files before release work.
2. Keep the check lightweight and safe for normal contributor machines.
3. Document the release preflight in public runbooks.
4. Preserve the existing CI and release workflows unchanged.

## Deliverables

1. Added `scripts/verify-local-preflight.sh`:
   - checks `.git` metadata for macOS dataless/offloaded files
   - checks tracked files for missing or dataless/offloaded state
   - verifies git object connectivity
   - requires a clean tracked worktree by default
   - supports `REQUIRE_CLEAN_WORKTREE=0` for validating the script during local edits
2. Updated `docs/RELEASE.md` with a macOS local storage preflight section and pre-tag commands.
3. Updated `docs/PUBLIC_RELEASE.md` to include the local preflight in repository privacy and validation checks.
4. Updated `CHANGELOG.md` with the release hygiene addition.

## Validation

Run from the repository root:

```bash
REQUIRE_CLEAN_WORKTREE=0 ./scripts/verify-local-preflight.sh
bash -n scripts/verify-local-preflight.sh
./scripts/verify-api-doc-commands.sh
npm run check
CARGO_TARGET_DIR=/tmp/900invoice-sprint070-target cargo check --manifest-path src-tauri/Cargo.toml
git diff --check -- scripts/verify-local-preflight.sh docs/RELEASE.md docs/PUBLIC_RELEASE.md CHANGELOG.md docs/sprints/sprint-070-release-hygiene-local-preflight.md
```

Expected result: the preflight passes on a hydrated checkout, shell syntax is valid, API docs parity passes, frontend type checks pass, Rust check passes, and the targeted diff has no whitespace errors.

## Decisions

1. Keep the preflight as an explicit script instead of adding it implicitly to every build command.
2. Make macOS dataless detection a local preflight concern rather than a CI check, because hosted CI checkouts are fresh and not affected by local iCloud/offload state.
3. Require clean tracked files by default for release usage, while allowing `REQUIRE_CLEAN_WORKTREE=0` during script development.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile selected or explicitly unchanged: `solo`.
- [x] Branch protection expectations reviewed against `docs/BRANCH_PROTECTION.md`.
- [x] Maintainer handoff expectations reviewed against `docs/MAINTAINER_CHECKLIST.md`.
- [x] Release or validation impact documented in this sprint record.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
