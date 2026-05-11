# Release Runbook

This runbook defines the current release process for 900Invoice.

---

## Release Model (Current)

Automated in GitHub Actions (`.github/workflows/release.yml`):

1. Run release quality gate checks.
2. Build a source tarball from the tagged commit.
3. Generate SHA-256 checksum file.
4. Publish artifacts to a GitHub Release for version tags (`v*`).
5. Enforce governance sprint-checklist parity against release diff scope.
6. Validate governance diff trace JSON against schema contract.
7. Publish governance diff trace summary and artifact for auditability.

Not yet automated:

1. Signed platform-specific binary bundles for Windows/macOS/Linux.
2. Notarization and code-signing pipelines.

---

## Pre-Tag Checklist

Before creating a release tag:

Use `docs/MAINTAINER_CHECKLIST.md` for maintainer governance/profile verification before this checklist.

1. Ensure `main` is green in CI.
2. Verify repository policy:
   - `./scripts/verify-repo-policy.sh 900Labs/900Invoice main`
   - (`STRICT=1` is default and required for releases)
   - If using non-`solo` governance, set matching profile env vars (for example `GOVERNANCE_PROFILE=small-team`).
3. Verify governance sprint checklist parity (same rule enforced in CI + release gate):
   - `./scripts/verify-governance-sprint-checklist.sh origin/main HEAD`
   - `REPORT_JSON_PATH=/tmp/release-governance-diff-context.json ./scripts/verify-governance-sprint-checklist.sh origin/main HEAD`
   - `./scripts/verify-governance-trace-json.sh /tmp/release-governance-diff-context.json`
4. Run local quality gate:
   - `./scripts/verify-api-doc-commands.sh`
   - `npm install`
   - `npm run check`
   - `CARGO_TARGET_DIR=/tmp/900invoice-target cargo check --manifest-path src-tauri/Cargo.toml`
   - `CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml`
   - `CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`
5. Confirm changelog and sprint documentation are updated.
6. For public repository visibility changes or public release prep, complete `docs/PUBLIC_RELEASE.md`.
7. Confirm the target version is reflected in release notes/changelog.

---

## Tagged Release Flow

From a clean local checkout on `main`:

```bash
git pull --ff-only origin main
git tag v1.0.1
git push origin v1.0.1
```

This triggers `.github/workflows/release.yml`.

Workflow outputs:

1. `900invoice-<version>-src.tar.gz`
2. `SHA256SUMS.txt`
3. `release-governance-diff-context-<tag>` artifact containing:
   - `release-governance-diff-context.txt`
   - `release-governance-diff-context.json`
4. Governance trace schema contract:
   - `docs/schemas/governance-diff-trace.schema.json`
   - compatibility rules enforced by validator:
     - accepts legacy `1.0.0` payloads (including payloads missing `schema_version`)
     - accepts current `1.1.0` payloads

Artifacts are uploaded to the workflow run and source bundle/checksum are attached to the GitHub Release.

The workflow summary also includes:

1. Selected governance diff base ref.
2. Governance checklist enforcement outcome.
3. Embedded governance diff context payload.
4. Location of machine-readable JSON governance trace.
5. Schema validation is enforced by `scripts/verify-governance-trace-json.sh`.
6. Governance artifact retention days and source (`workflow-override` or `global-default`).

---

## Manual Release Gate (Optional)

You can run the release gate manually from GitHub Actions using `workflow_dispatch`.

Optional input:

1. `tag` (for artifact naming, for example `v1.0.1`)

Use this for dry-runs before pushing a real version tag.

Release workflow governance verification defaults to `solo` unless repository variables are configured:

1. `GOVERNANCE_PROFILE`
2. `REQUIRED_APPROVING_REVIEW_COUNT` (optional override)
3. `REQUIRE_CODE_OWNER_REVIEWS` (optional override)
4. `REQUIRE_LAST_PUSH_APPROVAL` (optional override)
5. `STRICT_SPRINT_DOC_REFERENCE` (`0`/`1`; when `1`, all changed sprint docs must reference `docs/MAINTAINER_CHECKLIST.md`)
6. `GOVERNANCE_ARTIFACT_RETENTION_DAYS` (`1`-`90`; shared default `30`)
7. `RELEASE_GOVERNANCE_ARTIFACT_RETENTION_DAYS` (`1`-`90`; optional release-workflow override)

---

## Post-Release Checklist

1. Verify release artifacts and checksums on GitHub.
2. Validate that release notes were generated correctly.
3. Announce release and link checksums for integrity verification.
4. Track follow-up issues for packaging/signing if platform binaries are distributed separately.

---

## Future Hardening

Planned improvements:

1. Add cross-platform build matrix (Linux/macOS/Windows) with artifact uploads.
2. Add signing/notarization pipeline integration.
3. Add provenance/SBOM artifacts per release.
