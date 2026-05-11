# ADR 001: Tauri v2 as Desktop Framework

## Status: Accepted

## Date: 2026-03-01

## Context

900Invoice requires a cross-platform desktop application framework that produces small binaries, consumes minimal RAM at idle, and starts quickly. These are not merely convenience requirements — they are accessibility requirements.

The core user persona is a business owner or freelancer in a developing economy operating on hardware that is 3–5 years old, with 4 GB of RAM, running Windows 10 or Ubuntu, possibly sharing the machine with other users or applications. For this user:

- A 150 MB binary is a meaningful download cost on a metered 3G connection
- 300 MB of RAM idle consumption leaves little room for a browser and spreadsheet running simultaneously
- A 3-second startup time is frustrating on an HDD-based machine

We evaluated three frameworks: **Tauri v2**, **Electron**, and **Wails**.

### Tauri v2 vs Electron

| Metric | Tauri v2 | Electron | Notes |
|--------|----------|----------|-------|
| Binary Size | Small native bundle | 150–200+ MB | Electron bundles Chromium |
| RAM (idle) | Lower idle memory profile | 150–300+ MB | Electron runs a full browser process |
| Startup Time | Fast native-shell startup | 1–3 seconds | Electron has heavier initialization |
| Backend Language | Rust | Node.js/V8 | Rust is more performant for financial calculations |
| System WebView | Yes | No (bundled Chromium) | Tauri uses the OS's native WebView |

**Why Electron was rejected:** Binary size (150+ MB) and RAM consumption (150–300+ MB) are disqualifying for the target hardware profile. A tool that consumes 300 MB of RAM before the user opens a single invoice is not suitable for our users.

### Tauri v2 vs Wails

Wails is a similar Tauri alternative using Go for the backend. Both frameworks use the system WebView and produce small binaries.

| Criterion | Tauri v2 | Wails |
|-----------|----------|-------|
| Ecosystem maturity | Larger, more active | Smaller |
| Plugin ecosystem | Rich (Tauri plugins) | Limited |
| Consistency with 900 ecosystem | 900PDF and 900CRM already use Tauri | N/A |
| Community | Larger, good documentation | Smaller |

**Why Wails was not chosen:** We already use Tauri v2 in 900PDF and 900CRM. Maintaining a single framework across the ecosystem reduces onboarding friction for contributors and enables shared tooling.

## Decision

Use **Tauri v2** with a **Svelte 5** frontend and **Rust** backend.

- **Tauri v2**: Desktop shell, IPC bridge, file system access, and system dialogs; updater support can be added later through Tauri's plugin and configuration path
- **Svelte 5**: Frontend UI with Runes for reactive state — the smallest bundle of any major framework
- **Rust**: Backend business logic, database access, PDF rendering, scheduling

## Consequences

### Positive
- Smaller distributable footprint than Electron because Tauri does not bundle Chromium
- Lower idle memory profile than Electron because Tauri uses the system WebView
- Fast native-shell startup vs Electron's heavier Chromium initialization
- Native Rust backend enables fast financial calculations without JavaScript overhead
- Consistent with 900PDF and 900CRM, enabling shared contributor knowledge
- Strong security model: Tauri v2's capability-based permissions system

### Negative / Trade-offs
- System WebView means rendering differences between platforms (tested and managed)
- Rust has a steeper learning curve than JavaScript/Node.js for new contributors
- Building for all three platforms requires separate CI runners (mitigated by GitHub Actions matrix)
- Some Tauri v2 APIs changed from v1 — existing Tauri v1 tutorials may not apply

### Notes for Contributors
- The Tauri v2 documentation is at [v2.tauri.app](https://v2.tauri.app)
- Rust learning resources: [The Rust Book](https://doc.rust-lang.org/book/), [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- All Tauri commands are defined in `src-tauri/src/commands/` and registered in `src-tauri/src/lib.rs`
