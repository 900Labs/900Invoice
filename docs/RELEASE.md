# Release Runbook

This runbook defines the current release process for 900Invoice.

---

## Release Model (Current)

Automated in GitHub Actions (`.github/workflows/release.yml`):

1. Run release quality gate checks.
2. Build a source tarball from the tagged commit.
3. Generate SHA-256 checksum file.
4. Publish artifacts to a GitHub Release for version tags (`v*`).

Not yet automated:

1. Signed platform-specific binary bundles for Windows/macOS/Linux.
2. Notarization and code-signing pipelines.

---

## Pre-Tag Checklist

Before creating a release tag:

1. Ensure `main` is green in CI.
2. Run local quality gate:
   - `./scripts/verify-api-doc-commands.sh`
   - `npm install`
   - `npm run check`
   - `CARGO_TARGET_DIR=/tmp/900invoice-target cargo check --manifest-path src-tauri/Cargo.toml`
   - `CARGO_TARGET_DIR=/tmp/900invoice-target cargo test --manifest-path src-tauri/Cargo.toml`
   - `CARGO_TARGET_DIR=/tmp/900invoice-target cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings`
3. Confirm changelog and sprint documentation are updated.
4. Confirm the target version is reflected in release notes/changelog.

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

Artifacts are uploaded to the workflow run and attached to the GitHub Release.

---

## Manual Release Gate (Optional)

You can run the release gate manually from GitHub Actions using `workflow_dispatch`.

Optional input:

1. `tag` (for artifact naming, for example `v1.0.1`)

Use this for dry-runs before pushing a real version tag.

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
