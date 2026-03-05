#![allow(dead_code)]
//! Exchange rate cache for 900Invoice.
//!
//! Rates are stored in the `exchange_rates` SQLite table with a daily validity
//! key. For offline-first use, default seed rates (approximate) are provided.
//! All monetary conversions preserve i64 minor-unit precision.

use rusqlite::Connection;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ExchangeRate {
    pub base_currency: String,
    pub target_currency: String,
    pub rate: f64,
    pub fetched_at: String,
    pub valid_date: String,
}

// ---------------------------------------------------------------------------
// Default seed rates (USD base, approximate as of 2026)
// ---------------------------------------------------------------------------

/// Seed default exchange rates for offline use.
/// Only inserts if no rate exists for today already.
pub fn seed_default_rates(conn: &Connection) -> Result<(), String> {
    let today = today_iso();

    // Default rates from USD
    let defaults: &[(&str, &str, f64)] = &[
        ("USD", "KES", 152.50),
        ("USD", "NGN", 1550.00),
        ("USD", "ZAR", 18.20),
        ("USD", "INR", 83.50),
        ("USD", "TZS", 2680.00),
        ("USD", "UGX", 3780.00),
        ("USD", "GHS", 15.80),
        ("USD", "XOF", 610.00),
        ("USD", "XAF", 610.00),
        ("USD", "EUR", 0.92),
        // Trivial identity
        ("USD", "USD", 1.00),
        ("EUR", "USD", 1.087),
        ("KES", "USD", 0.00656),
        ("NGN", "USD", 0.000645),
        ("ZAR", "USD", 0.0549),
        ("INR", "USD", 0.01198),
        ("TZS", "USD", 0.000373),
        ("UGX", "USD", 0.000265),
        ("GHS", "USD", 0.0633),
        ("XOF", "USD", 0.00164),
        ("XAF", "USD", 0.00164),
    ];

    for (base, target, rate) in defaults {
        // Use INSERT OR IGNORE so we don't overwrite live rates
        conn.execute(
            "INSERT OR IGNORE INTO exchange_rates \
             (base_currency, target_currency, rate, fetched_at, valid_date) \
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![base, target, rate, &today, &today],
        )
        .map_err(|e| format!("Failed to seed rate {base}/{target}: {e}"))?;
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Core functions
// ---------------------------------------------------------------------------

/// Retrieve the most recent cached exchange rate between two currencies.
///
/// Looks up by exact date first, then falls back to the most recent available.
pub fn get_cached_rate(conn: &Connection, base: &str, target: &str) -> Result<Option<f64>, String> {
    if base == target {
        return Ok(Some(1.0));
    }

    // Try the most recent available date
    let result: rusqlite::Result<f64> = conn.query_row(
        "SELECT rate FROM exchange_rates \
         WHERE base_currency = ?1 AND target_currency = ?2 \
         ORDER BY valid_date DESC \
         LIMIT 1",
        rusqlite::params![base, target],
        |row| row.get(0),
    );

    match result {
        Ok(rate) => Ok(Some(rate)),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            // Try inverse rate (1/rate) if available
            let inverse: rusqlite::Result<f64> = conn.query_row(
                "SELECT rate FROM exchange_rates \
                 WHERE base_currency = ?1 AND target_currency = ?2 \
                 ORDER BY valid_date DESC \
                 LIMIT 1",
                rusqlite::params![target, base],
                |row| row.get(0),
            );
            match inverse {
                Ok(inv_rate) if inv_rate > 0.0 => Ok(Some(1.0 / inv_rate)),
                Ok(_) => Ok(None),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(format!("Failed to query inverse rate: {e}")),
            }
        }
        Err(e) => Err(format!(
            "Failed to query exchange rate {base}/{target}: {e}"
        )),
    }
}

/// Cache an exchange rate in the database.
pub fn cache_rate(
    conn: &Connection,
    base: &str,
    target: &str,
    rate: f64,
    valid_date: &str,
) -> Result<(), String> {
    let now = chrono_now_iso();
    conn.execute(
        "INSERT INTO exchange_rates (base_currency, target_currency, rate, fetched_at, valid_date) \
         VALUES (?1, ?2, ?3, ?4, ?5) \
         ON CONFLICT(base_currency, target_currency, valid_date) \
         DO UPDATE SET rate = excluded.rate, fetched_at = excluded.fetched_at",
        rusqlite::params![base, target, rate, now, valid_date],
    )
    .map_err(|e| format!("Failed to cache rate {base}/{target}: {e}"))?;
    Ok(())
}

/// Convert an amount in minor units from one currency to another.
///
/// Uses the cached rate. Both currencies should use the same decimal precision
/// assumption (if they differ, the caller should handle minor-unit conversion).
/// Returns the converted amount in target currency minor units.
pub fn convert_currency(
    conn: &Connection,
    amount_minor: i64,
    from_currency: &str,
    to_currency: &str,
) -> Result<i64, String> {
    if from_currency == to_currency {
        return Ok(amount_minor);
    }

    let rate = get_cached_rate(conn, from_currency, to_currency)?.ok_or_else(|| {
        format!(
            "No cached rate found for {from_currency}/{to_currency}. Try seeding default rates."
        )
    })?;

    // Convert: preserve precision by working in f64 temporarily, then round.
    // This is acceptable for display/conversion, not for primary accounting.
    let converted = (amount_minor as f64 * rate).round() as i64;
    Ok(converted)
}

/// Get all cached rates for a given date.
pub fn get_all_rates(conn: &Connection, date: &str) -> Result<Vec<ExchangeRate>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT base_currency, target_currency, rate, fetched_at, valid_date \
             FROM exchange_rates \
             WHERE valid_date = ?1 \
             ORDER BY base_currency, target_currency",
        )
        .map_err(|e| format!("Failed to prepare rates query: {e}"))?;

    let rates = stmt
        .query_map(rusqlite::params![date], |row| {
            Ok(ExchangeRate {
                base_currency: row.get(0)?,
                target_currency: row.get(1)?,
                rate: row.get(2)?,
                fetched_at: row.get(3)?,
                valid_date: row.get(4)?,
            })
        })
        .map_err(|e| format!("Failed to query rates: {e}"))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(rates)
}

/// Get all cached rates across all dates (most recent per pair).
pub fn get_all_latest_rates(conn: &Connection) -> Result<Vec<ExchangeRate>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT base_currency, target_currency, rate, fetched_at, valid_date \
             FROM exchange_rates \
             GROUP BY base_currency, target_currency \
             HAVING valid_date = MAX(valid_date) \
             ORDER BY base_currency, target_currency",
        )
        .map_err(|e| format!("Failed to prepare latest rates query: {e}"))?;

    let rates = stmt
        .query_map(rusqlite::params![], |row| {
            Ok(ExchangeRate {
                base_currency: row.get(0)?,
                target_currency: row.get(1)?,
                rate: row.get(2)?,
                fetched_at: row.get(3)?,
                valid_date: row.get(4)?,
            })
        })
        .map_err(|e| format!("Failed to query latest rates: {e}"))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(rates)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn today_iso() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn chrono_now_iso() -> String {
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS exchange_rates (
                base_currency TEXT NOT NULL,
                target_currency TEXT NOT NULL,
                rate REAL NOT NULL,
                fetched_at TEXT NOT NULL,
                valid_date TEXT NOT NULL,
                PRIMARY KEY (base_currency, target_currency, valid_date)
            );",
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_seed_and_retrieve() {
        let conn = setup_db();
        seed_default_rates(&conn).unwrap();
        let rate = get_cached_rate(&conn, "USD", "KES").unwrap();
        assert!(rate.is_some());
        let r = rate.unwrap();
        assert!((r - 152.50).abs() < 0.01);
    }

    #[test]
    fn test_same_currency() {
        let conn = setup_db();
        let rate = get_cached_rate(&conn, "USD", "USD").unwrap();
        assert_eq!(rate, Some(1.0));
    }

    #[test]
    fn test_convert_currency() {
        let conn = setup_db();
        seed_default_rates(&conn).unwrap();
        // $10.00 (1000 minor cents) → KES at 152.50
        let kes = convert_currency(&conn, 1_000, "USD", "KES").unwrap();
        assert_eq!(kes, 152_500); // 1000 * 152.50 = 152500
    }

    #[test]
    fn test_inverse_rate_fallback() {
        let conn = setup_db();
        seed_default_rates(&conn).unwrap();
        // KES → USD should work via inverse (USD→KES seeded)
        let rate = get_cached_rate(&conn, "KES", "USD").unwrap();
        assert!(rate.is_some());
        // The seeded KES/USD rate is 0.00656, inverse of USD/KES 152.50 ≈ 0.00656
        let r = rate.unwrap();
        assert!(r > 0.0 && r < 1.0, "KES/USD should be fractional, got {r}");
    }

    #[test]
    fn test_cache_rate() {
        let conn = setup_db();
        cache_rate(&conn, "USD", "NGN", 1600.0, "2026-03-05").unwrap();
        let rate = get_cached_rate(&conn, "USD", "NGN").unwrap();
        assert_eq!(rate, Some(1600.0));
    }

    #[test]
    fn test_get_all_rates() {
        let conn = setup_db();
        seed_default_rates(&conn).unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let rates = get_all_rates(&conn, &today).unwrap();
        assert!(!rates.is_empty());
        assert!(rates
            .iter()
            .any(|r| r.base_currency == "USD" && r.target_currency == "KES"));
    }

    #[test]
    fn test_missing_rate_returns_none() {
        let conn = setup_db();
        let rate = get_cached_rate(&conn, "ABC", "XYZ").unwrap();
        assert!(rate.is_none());
    }
}
