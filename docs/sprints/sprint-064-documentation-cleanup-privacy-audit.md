# Sprint 064 — Documentation Cleanup and Privacy Audit

Status: Completed
Date: 2026-05-20
Owner: 900 Labs

## Context

The repository is being prepared for public open-source visibility. Sprint 063 completed the first public-release documentation cleanup. This sprint is a follow-up pass focused on human-readable documentation ordering, stale contributor instructions, and privacy checks for personal identifiers or local machine paths.

No sprint records beyond Sprint 063 existed before this sprint was added, so Sprint 064 is the next documented sprint.

## Goals

1. Confirm whether any later sprint records already exist.
2. Add a sorted documentation index for GitHub readers.
3. Refresh stale contributor and i18n instructions.
4. Re-run privacy checks for local paths, personal names, hostnames, and operating-system metadata.
5. Keep the cleanup documentation-only unless validation discovers a repository hygiene issue.

## Deliverables

1. Added `docs/README.md` as a sorted documentation index.
2. Updated `README.md` to link the documentation index.
3. Updated `CONTRIBUTING.md` for current Rust/Node prerequisites, repository casing, command count, Svelte rune store filename, tax-rate seeding path, and currency-extension paths.
4. Updated `docs/I18N.md` to reference `src/stores/i18nStore.svelte.ts`.
5. Updated `docs/PUBLIC_RELEASE.md` with a stronger privacy and stale-doc scan baseline.
6. Updated `docs/DEPLOYMENT.md` to keep macOS signing examples organization-generic.
7. Removed ignored local operating-system metadata files from the checkout.

## Validation

Run from the repository root:

```bash
rg --files docs/sprints | sort -V | tail -n 10
git grep -n -I -E '(/Users/[^ /]+|/home/[^ /]+|C:\\Users\\[^\\]+|Desktop/[A-Za-z0-9._-]+)' -- . ':(exclude)package-lock.json' ':(exclude)docs/PUBLIC_RELEASE.md'
LOCAL_HOSTNAME="$(hostname -s 2>/dev/null || true)"
test -z "$LOCAL_HOSTNAME" || git grep -n -I -F "$LOCAL_HOSTNAME" -- . ':(exclude)package-lock.json' ':(exclude)docs/PUBLIC_RELEASE.md'
git ls-files | rg '(^|/)\.DS_Store$|(^|/)Thumbs\.db$|(^|/)desktop\.ini$'
find . \( -name .DS_Store -o -name Thumbs.db -o -name desktop.ini \) -print
rg -n -F \
  -e 'Rust 1.75' \
  -e 'Node.js 18' \
  -e 'SUPPORTED_LANGUAGES' \
  -e 'src/i18n/index.ts' \
  -e 'src/stores/i18nStore.ts' \
  -e '~45 commands' \
  -e 'sequence_counters' \
  -e 'services/numbering.rs' \
  -e 'src-tauri/src/services/tax.rs' \
  -e 'src-tauri/src/models/currency.rs' \
  README.md CONTRIBUTING.md docs --glob '!docs/sprints/**' --glob '!docs/PUBLIC_RELEASE.md'
git diff --check -- README.md CONTRIBUTING.md CHANGELOG.md docs/README.md docs/PUBLIC_RELEASE.md docs/DEPLOYMENT.md docs/I18N.md docs/sprints/sprint-064-documentation-cleanup-privacy-audit.md
```

Expected result: Sprint 064 is the latest sprint record; privacy and metadata scans report no tracked or local hits; stale-doc scan reports no hits outside excluded historical sprint/public-release checklist files; documentation whitespace check passes.

## Decisions

1. Keep GitHub organization and repository URLs because the user explicitly allows GitHub username/organization references.
2. Keep public 900 Labs product and ecosystem references because they are project identity, not personal identifiers.
3. Keep historical sprint records unchanged unless they contain personal identifiers; historical implementation notes are allowed to remain as audit history.

<!-- MAINTAINER_CHECKLIST_COMPLETION:BEGIN -->
- [x] Governance profile selected or explicitly unchanged: `solo`.
- [x] Branch protection expectations reviewed against `docs/BRANCH_PROTECTION.md`.
- [x] Maintainer handoff expectations reviewed against `docs/MAINTAINER_CHECKLIST.md`.
- [x] Release or validation impact documented in this sprint record.
<!-- MAINTAINER_CHECKLIST_COMPLETION:END -->
