# Deployment Guide

This guide covers building 900Invoice from source on Windows, macOS, and Linux, plus distribution options for maintainers.

The current automated release workflow publishes source archives and checksums. Signed platform-specific installers, notarization, and auto-update delivery are future release hardening items.

---

## Prerequisites

### All Platforms
- **Rust 1.88+**: [rustup.rs](https://rustup.rs)
- **Node.js 20.19+, 22.12+, or 24+**: [nodejs.org](https://nodejs.org)
- **Tauri CLI v2**: required for `cargo tauri` commands
  ```bash
  cargo install tauri-cli --version "^2"
  ```

### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y \
  libgtk-3-dev \
  libwebkit2gtk-4.1-dev \
  librsvg2-dev \
  patchelf \
  libssl-dev \
  libappindicator3-dev
```

### Linux (Fedora/RHEL)
```bash
sudo dnf install -y \
  gtk3-devel \
  webkit2gtk4.1-devel \
  librsvg2-devel \
  patchelf \
  openssl-devel
```

### Linux (Arch)
```bash
sudo pacman -S \
  gtk3 \
  webkit2gtk-4.1 \
  librsvg \
  patchelf \
  openssl
```

### macOS
```bash
xcode-select --install
```

Xcode Command Line Tools provides the compiler, linker, and system libraries needed. No additional packages are required for basic builds.

### Windows
1. Install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
   - Select "Desktop development with C++" workload
2. WebView2 is pre-installed on Windows 10 (version 1803+) and Windows 11
3. For older systems, download [WebView2 Evergreen Runtime](https://developer.microsoft.com/microsoft-edge/webview2/)

---

## Building from Source

### Development Build (with hot-reload)

```bash
git clone https://github.com/900Labs/900Invoice.git
cd 900Invoice
npm install
cargo tauri dev
```

This starts the application in development mode. Changes to Svelte files reload instantly in the window. Changes to Rust files trigger an incremental recompile (typically 5–30 seconds depending on your machine).

### Production Build

```bash
npm install
cargo tauri build
```

Local `cargo tauri build` output is written under `src-tauri/target/release/bundle/`. Exact artifacts vary by host OS, target architecture, and local Tauri bundler configuration. Typical local outputs include:

| Platform | Output Location | File |
|----------|----------------|------|
| Linux | `bundle/appimage/` | `900invoice_1.0.0_amd64.AppImage` |
| Linux | `bundle/deb/` | `900invoice_1.0.0_amd64.deb` |
| Linux | `bundle/rpm/` | `900invoice-1.0.0-1.x86_64.rpm` |
| macOS | `bundle/dmg/` | `900Invoice_1.0.0_x64.dmg` |
| macOS | `bundle/macos/` | `900Invoice.app` |
| Windows | `bundle/msi/` | `900Invoice_1.0.0_x64_en-US.msi` |
| Windows | `bundle/nsis/` | `900Invoice_1.0.0_x64-setup.exe` |

---

## Platform-Specific Notes

### Linux

**AppImage** is the most portable format. Users can run it directly without installation:
```bash
chmod +x 900invoice_1.0.0_amd64.AppImage
./900invoice_1.0.0_amd64.AppImage
```

**Debian/Ubuntu package** installation:
```bash
sudo dpkg -i 900invoice_1.0.0_amd64.deb
```

**Data location on Linux**: `~/.local/share/com.900labs.invoice/900invoice.db`

**System WebKit requirement**: Tauri v2 requires `webkit2gtk-4.1` on Linux. Most modern distributions include this. Ubuntu 22.04+ and Fedora 38+ have it by default.

For users on older distributions, advise:
- Ubuntu 20.04: Install webkit2gtk-4.1 from the official backports PPA
- Debian Bullseye (11): May require `webkit2gtk-4.0` compatibility; Tauri v2 support is limited on Debian 11

### macOS

**Data location on macOS**: `~/Library/Application Support/com.900labs.invoice/900invoice.db`

**Apple Silicon (M1/M2/M3)**: Build natively on ARM for best performance:
```bash
# Native ARM build
cargo tauri build --target aarch64-apple-darwin

# Universal binary (Intel + ARM) — larger but works on all Macs
cargo tauri build --target universal-apple-darwin
```

The current CI/CD release workflow does not build or publish macOS binaries. Maintainers can build universal binaries locally with the command above until platform build jobs are added.

**macOS Gatekeeper**: Unsigned `.app` bundles will be blocked by Gatekeeper on macOS 12+. Users can bypass this with:
```bash
xattr -rd com.apple.quarantine /Applications/900Invoice.app
```

For production releases, code signing is strongly recommended (see below).

### Windows

**Data location on Windows**: `%APPDATA%\com.900labs.invoice\900invoice.db` (e.g., `C:\Users\Alice\AppData\Roaming\com.900labs.invoice\`)

**MSI vs NSIS installer**: The MSI installer is preferred for enterprise deployment (Group Policy, SCCM). The NSIS installer (`-setup.exe`) is preferred for individual user installs.

**Windows SmartScreen**: Unsigned executables trigger a SmartScreen warning. Users can click "More info" → "Run anyway". For production, code signing is strongly recommended.

---

## Code Signing

### macOS Code Signing

Code signing is required to distribute on macOS without Gatekeeper warnings.

**Requirements:**
- Apple Developer account ($99/year)
- "Developer ID Application" certificate
- Notarization (required for macOS 10.15+)

**Environment variables for CI:**
```
APPLE_CERTIFICATE             # Base64-encoded .p12 certificate
APPLE_CERTIFICATE_PASSWORD    # Certificate password
APPLE_SIGNING_IDENTITY        # "Developer ID Application: Your Name (TEAM_ID)"
APPLE_ID                      # Your Apple ID email
APPLE_PASSWORD                # App-specific password (from appleid.apple.com)
APPLE_TEAM_ID                 # Your Apple Team ID
```

**tauri.conf.json configuration:**
```json
{
  "bundle": {
    "macOS": {
      "signingIdentity": "Developer ID Application: 900 Labs (TEAM_ID)",
      "providerShortName": "TEAM_ID",
      "entitlements": "entitlements.plist"
    }
  }
}
```

### Windows Code Signing

**Requirements:**
- Code signing certificate from a trusted CA (DigiCert, Sectigo, GlobalSign, etc.)
- Certificate in PFX format

**Environment variables for CI:**
```
WINDOWS_CERTIFICATE    # Base64-encoded .pfx certificate
WINDOWS_CERTIFICATE_PASSWORD
```

**tauri.conf.json configuration:**
```json
{
  "bundle": {
    "windows": {
      "certificateThumbprint": null,
      "digestAlgorithm": "sha256",
      "timestampUrl": "http://timestamp.digicert.com"
    }
  }
}
```

---

## Auto-Update Configuration

900Invoice does not currently enable Tauri auto-update in `tauri.conf.json` or ship an updater plugin dependency. The outline below is retained for future hardening work if signed platform binaries are added.

### 1. Set up an update endpoint

The update endpoint must return JSON describing the latest release. Host this file on a stable URL (e.g., GitHub Pages, your own CDN):

```json
{
  "version": "1.0.1",
  "notes": "Bug fixes and improvements",
  "pub_date": "2026-04-01T00:00:00Z",
  "platforms": {
    "linux-x86_64": {
      "signature": "...",
      "url": "https://github.com/900Labs/900Invoice/releases/download/v1.0.1/900invoice_1.0.1_amd64.AppImage.tar.gz"
    },
    "darwin-aarch64": {
      "signature": "...",
      "url": "https://github.com/900Labs/900Invoice/releases/download/v1.0.1/900Invoice_1.0.1_aarch64.app.tar.gz"
    },
    "darwin-x86_64": {
      "signature": "...",
      "url": "https://github.com/900Labs/900Invoice/releases/download/v1.0.1/900Invoice_1.0.1_x64.app.tar.gz"
    },
    "windows-x86_64": {
      "signature": "...",
      "url": "https://github.com/900Labs/900Invoice/releases/download/v1.0.1/900Invoice_1.0.1_x64_en-US.msi.zip"
    }
  }
}
```

### 2. Configure tauri.conf.json

```json
{
  "app": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://releases.900labs.com/900invoice/{{target}}/{{arch}}/{{current_version}}"
      ],
      "dialog": true,
      "pubkey": "YOUR_PUBLIC_KEY_HERE"
    }
  }
}
```

### 3. Generate signing keys

```bash
cargo tauri signer generate -w ~/.tauri/900invoice.key
# This outputs a public key — add it to tauri.conf.json
# Keep the private key SECRET — add it to your CI secrets as TAURI_PRIVATE_KEY
```

---

## Distribution Channels

### GitHub Releases (Primary)

The release gate pipeline (`.github/workflows/release.yml`) runs on version tags and publishes release metadata artifacts (source tarball + checksum):

```bash
git tag v1.0.1
git push origin v1.0.1
```

For the current release model and checklist, see `docs/RELEASE.md`.

### Linux Package Managers

**Flatpak** (recommended for wide Linux distribution):
1. Create a Flatpak manifest (`com.900labs.Invoice.yml`)
2. Submit to [Flathub](https://flathub.org) for inclusion in the official repository
3. Users install with: `flatpak install flathub com.900labs.Invoice`

**Snap**:
1. Create a `snapcraft.yaml`
2. Publish to the [Snap Store](https://snapcraft.io)

**AUR (Arch User Repository)** — community maintained:
If you use Arch Linux and want to maintain an AUR package for 900Invoice, this is a valuable contribution. The package name should be `900invoice` or `900invoice-bin`.

### macOS Homebrew

For macOS distribution via Homebrew:
```ruby
# Formula at homebrew-cask/Casks/900invoice.rb
cask "900invoice" do
  version "1.0.0"
  sha256 "..." # sha256 of the .dmg

  url "https://github.com/900Labs/900Invoice/releases/download/v#{version}/900Invoice_#{version}_x64.dmg"

  name "900Invoice"
  desc "Enterprise-grade invoicing for developing economies"
  homepage "https://www.900labs.com"

  app "900Invoice.app"
end
```

---

## CI/CD Pipeline

The GitHub Actions workflows currently handle quality checks and release-gate automation:
- `.github/workflows/ci.yml` — runs on every push and PR
- `.github/workflows/release.yml` — runs on version tags (`v*`) and manual dispatch

### Required Secrets for CI

Configure these in your GitHub repository Settings → Secrets and Variables → Actions:

| Secret | Required For |
|--------|-------------|
| `GITHUB_TOKEN` | Built-in token used for creating/updating GitHub releases |
| `TAURI_PRIVATE_KEY` | Optional future auto-update signature pipeline |
| `TAURI_KEY_PASSWORD` | Optional future auto-update signature pipeline |
| `APPLE_CERTIFICATE` | Optional future macOS code signing pipeline |
| `APPLE_CERTIFICATE_PASSWORD` | Optional future macOS code signing pipeline |
| `APPLE_SIGNING_IDENTITY` | Optional future macOS code signing pipeline |
| `APPLE_ID` | Optional future macOS notarization pipeline |
| `APPLE_PASSWORD` | Optional future macOS notarization pipeline |
| `APPLE_TEAM_ID` | Optional future macOS notarization pipeline |
| `WINDOWS_CERTIFICATE` | Optional future Windows code signing pipeline |
| `WINDOWS_CERTIFICATE_PASSWORD` | Optional future Windows code signing pipeline |

---

## Database Location Reference

| Platform | Path |
|----------|------|
| Linux | `~/.local/share/com.900labs.invoice/900invoice.db` |
| macOS | `~/Library/Application Support/com.900labs.invoice/900invoice.db` |
| Windows | `%APPDATA%\com.900labs.invoice\900invoice.db` |

The database path is determined by Tauri's `app_data_dir()` function, which resolves to the platform data directory plus the bundle identifier from `src-tauri/tauri.conf.json`.

---

## Troubleshooting

### Build fails: "webkit2gtk not found" (Linux)

```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev
# Note: it must be 4.1, not 4.0
```

### Build fails on macOS: "xcrun not found"

```bash
xcode-select --install
# If already installed, reset:
sudo xcode-select --reset
```

### AppImage won't run: "error while loading shared libraries"

This usually means a system library dependency is missing. Try:
```bash
./900invoice_1.0.0_amd64.AppImage --appimage-extract
cd squashfs-root
./AppRun
# Check error output for the missing library name
```

### Windows: "VCRUNTIME140.dll not found"

Install the [Microsoft Visual C++ Redistributable](https://docs.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist).

### Application won't start after update

If the application can still reach Settings, restore from a backup:
1. Open Settings
2. Use Restore Database
3. Select a backup file

If startup is blocked before Settings is available, move the database file out of the app data directory listed above and start the application again. The application will create a new empty database. Data from the moved database will remain unavailable unless it can be recovered or restored from a backup.
