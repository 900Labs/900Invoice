use crate::db;
use rusqlite::Connection;

#[derive(Debug, Clone, PartialEq)]
pub struct ExchangeRateSnapshot {
    pub rate_to_usd: Option<f64>,
    pub valid_date: Option<String>,
}

pub fn snapshot_to_usd(
    conn: &Connection,
    currency_code: &str,
    issue_date: &str,
) -> Result<ExchangeRateSnapshot, String> {
    let currency = currency_code.trim().to_uppercase();
    let effective_date = if issue_date.trim().is_empty() {
        chrono::Local::now().format("%Y-%m-%d").to_string()
    } else {
        issue_date.to_string()
    };

    if currency == "USD" {
        return Ok(ExchangeRateSnapshot {
            rate_to_usd: Some(1.0),
            valid_date: Some(effective_date),
        });
    }

    if let Some(rate) =
        db::queries::exchange_rates::get_rate(conn, &currency, "USD", &effective_date)
            .map_err(|e| e.to_string())?
    {
        return Ok(ExchangeRateSnapshot {
            rate_to_usd: Some(rate.rate),
            valid_date: Some(rate.valid_date),
        });
    }

    if let Some(rate) =
        db::queries::exchange_rates::get_rate(conn, "USD", &currency, &effective_date)
            .map_err(|e| e.to_string())?
    {
        if rate.rate > 0.0 {
            return Ok(ExchangeRateSnapshot {
                rate_to_usd: Some(1.0 / rate.rate),
                valid_date: Some(rate.valid_date),
            });
        }
    }

    Ok(ExchangeRateSnapshot {
        rate_to_usd: None,
        valid_date: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::exchange_rate_sync;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().expect("in-memory db");
        conn.execute_batch(
            "CREATE TABLE exchange_rates (
                base_currency TEXT NOT NULL,
                target_currency TEXT NOT NULL,
                rate REAL NOT NULL,
                fetched_at TEXT NOT NULL,
                valid_date TEXT NOT NULL,
                PRIMARY KEY (base_currency, target_currency, valid_date)
            );",
        )
        .expect("exchange table");
        conn
    }

    #[test]
    fn usd_snapshot_uses_identity_rate() {
        let conn = setup_db();
        let snapshot = snapshot_to_usd(&conn, "USD", "2026-05-11").expect("snapshot");
        assert_eq!(
            snapshot,
            ExchangeRateSnapshot {
                rate_to_usd: Some(1.0),
                valid_date: Some("2026-05-11".to_string()),
            }
        );
    }

    #[test]
    fn non_usd_snapshot_uses_cached_inverse_to_usd() {
        let conn = setup_db();
        exchange_rate_sync::seed_default_rates(&conn).expect("seed rates");
        let snapshot = snapshot_to_usd(&conn, "KES", "9999-12-31").expect("snapshot");

        assert!(snapshot.rate_to_usd.unwrap_or_default() > 0.0);
        assert!(snapshot.rate_to_usd.unwrap_or_default() < 1.0);
        assert!(snapshot.valid_date.is_some());
    }
}
