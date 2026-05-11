# ADR 004: Offline-First Architecture

## Status: Accepted

## Date: 2026-03-01

## Context

Most modern invoicing applications are cloud-first: data lives on a server, the application is a web client, and internet connectivity is a hard requirement. This model fails for a large portion of our target users.

### Internet Access in Developing Economies (2025)

- Nigeria: ~55% internet penetration; average broadband speed 10–20 Mbps in urban areas, but frequent outages and high cost per GB
- Kenya: ~40% internet penetration; mobile internet dominant (M-Pesa is offline-capable for a reason)
- Tanzania/Uganda: 25–35% internet penetration; rural areas frequently unconnected
- India: 50%+ penetration, but quality is highly variable; rural areas often 2G or 3G only

For a freelancer running a consulting business from Nairobi, being unable to generate an invoice because the internet is down is not a minor inconvenience — it is a loss of income. The client needs the invoice today; the payment process cannot start until the invoice is issued.

### Problems with Cloud-First Invoicing for Our Users

1. **Cost**: Cloud subscriptions are priced for wealthy-country purchasing power
2. **Reliability**: Internet connectivity is not guaranteed during invoice creation
3. **Speed**: Round-trip latency to cloud servers in US/Europe adds noticeable delay on mobile networks
4. **Privacy**: Financial data crossing international borders raises data sovereignty concerns
5. **Vendor lock-in**: If the SaaS provider goes bankrupt or changes pricing, all data may be inaccessible

### Why Not a Local Web App (localhost)?

A local web server approach (e.g., Django/Rails running on localhost) avoids most cloud issues but introduces:
- Complex installation (Python/Ruby runtime, database setup, web server)
- Multiple processes to manage
- Port conflicts
- No native file system access (file picker dialogs, PDF viewer integration)

### Why Not a PWA?

Progressive Web Apps require a browser and work well online. Offline PWAs exist but:
- Complex to implement correctly (service workers, IndexedDB limitations)
- No access to native file system for PDF export
- Requires a browser to be running
- Browsers may clear local storage under memory pressure

## Decision

900Invoice is **100% offline-first**: all data is stored in a local SQLite database and all processing happens on the user's device. No network request is required for any core function.

**Functions that are offline-capable:**
- Creating, editing, and finalizing invoices
- PDF generation
- All CRUD operations for clients, products, tax rates
- Reports and dashboard
- Database backup and restore
- CSV import/export

**Functions that require internet (optional, gracefully degrade):**
- Exchange rate updates (falls back to cached rates; app includes sensible defaults)
- Future: multi-device sync (not in v1.0.0)

**Data storage:**
- Single SQLite file: `{APP_DATA_DIR}/900invoice.db`
- All operations go through the Rust backend via Tauri IPC
- No `localStorage`, no `IndexedDB`, no browser storage

**Sync readiness:**
While v1.0.0 is offline-only, the schema includes a `changelog` table that records every write operation with a device ID and timestamp. This is the foundation for future optional sync (cloud or peer-to-peer) without requiring schema changes.

## Consequences

### Positive
- Works without any internet connection — always, for all core functions
- No subscription fees required
- Data stays on the user's device — full data sovereignty
- No server infrastructure to maintain (zero operational cost)
- Fast: no network latency for any operation
- Private: financial data never leaves the device

### Negative / Trade-offs
- No automatic cross-device sync in v1.0.0 (planned for a future release)
- Database backup is manual; users are responsible for creating and storing their own backup files
- Exchange rates require manual update or occasional internet access; rates can become stale

### Backup Strategy

Because the user is responsible for their data, 900Invoice:
1. Provides manual backup export to a selected local JSON file via Settings → Backup Database
2. Provides additive restore from a selected JSON backup via Settings → Restore Database
3. Stores live application data in a standard local SQLite file at `{APP_DATA_DIR}/900invoice.db`
4. Keeps the Settings backup format as a versioned JSON data snapshot, not a raw SQLite database copy

### Notes for Contributors

Never add any network requests that are required for core functionality. Optional network features (exchange rate updates) must:
1. Fail gracefully with a user-visible but non-blocking error
2. Fall back to the most recent cached value
3. Never block invoice creation or PDF generation
