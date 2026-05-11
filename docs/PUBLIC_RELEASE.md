# Public Release Checklist

Use this checklist before making the repository public or publishing a release intended for public consumption.

## Repository Privacy

Run these checks from a clean checkout on `main`:

```bash
git status --short --branch
git ls-files | rg '(^|/)\.DS_Store$|(^|/)Thumbs\.db$|(^|/)desktop\.ini$'
git grep -n -I -E '(/Users/|/home/[^ /]+|C:\\Users\\|MacBook)' -- . ':(exclude)package-lock.json' ':(exclude)docs/PUBLIC_RELEASE.md'
```

Expected result:

1. The worktree is clean.
2. No tracked operating-system metadata files are reported.
3. No personal local paths, usernames, hostnames, or machine-specific development paths are reported.

Notes:

1. `src-tauri/tauri.conf.json` may contain a localhost Tauri development URL; this is a framework dev-server setting, not a personal identifier.
2. Ignored local files such as `.DS_Store`, `dist/`, `node_modules/`, `.tmp/`, and `/pdf/` must remain untracked.

## Public-Facing Files

Confirm these files are present and current:

1. `README.md` explains the product, source-build requirements, release model, data storage, and documentation map.
2. `CONTRIBUTING.md` explains setup, coding standards, sprint process, i18n, tax-rate contributions, and PR expectations.
3. `SECURITY.md` explains vulnerability reporting.
4. `CODE_OF_CONDUCT.md` defines community conduct expectations.
5. `LICENSE` contains the Apache-2.0 license.
6. `CHANGELOG.md` includes user-visible changes since the last release.

## Documentation Sanity

Check active documentation for stale implementation names before publishing:

```bash
rg -n -F -e 'Rust 1.75+' -e 'Node.js 18+' -e 'SUPPORTED_LANGUAGES' -e 'src/i18n/index.ts' -e 'sequence_counters' -e 'services/numbering.rs' README.md CONTRIBUTING.md docs --glob '!docs/sprints/**' --glob '!docs/PUBLIC_RELEASE.md'
```

Expected result: no matches.

If the README feature list changes, verify the implementation and docs together:

1. Source-build prerequisites match `src-tauri/Cargo.toml`, `Cargo.lock`, `package.json`, and `package-lock.json`.
2. Feature bullets map to shipped UI, Tauri command, or service code paths.
3. Documentation links point to existing files.

## Validation

Run the public release validation baseline:

```bash
./scripts/verify-api-doc-commands.sh
npm run check
SMOKE_PROFILE=full CARGO_TARGET_DIR=/tmp/900invoice-target-check ./scripts/verify-runtime-smoke.sh
git diff --check
```

The full smoke script includes the frontend production build, Rust check, Rust tests, Rust doc tests, and clippy with warnings denied.

## GitHub Readiness

Before switching visibility or publishing a tag:

1. Confirm the default branch is `main`.
2. Confirm branch protection expectations in `docs/BRANCH_PROTECTION.md`.
3. Confirm CI is green for `main`.
4. Confirm the latest release workflow behavior in `docs/RELEASE.md`.
5. Confirm no private issue, PR, or discussion content is referenced from public docs.
