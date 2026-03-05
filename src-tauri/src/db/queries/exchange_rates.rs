use crate::models::exchange_rate::ExchangeRate;
use rusqlite::{params, Connection, Result};

fn row_to_rate(row: &rusqlite::Row<'_>) -> Result<ExchangeRate> {
    Ok(ExchangeRate {
        base_currency: row.get(0)?,
        target_currency: row.get(1)?,
        rate: row.get(2)?,
        fetched_at: row.get(3)?,
        valid_date: row.get(4)?,
    })
}

pub fn get_rate(
    conn: &Connection,
    base: &str,
    target: &str,
    date: &str,
) -> Result<Option<ExchangeRate>> {
    let mut stmt = conn.prepare(
        "SELECT base_currency, target_currency, rate, fetched_at, valid_date
         FROM exchange_rates
         WHERE base_currency=?1 AND target_currency=?2 AND valid_date<=?3
         ORDER BY valid_date DESC LIMIT 1",
    )?;
    let mut rows = stmt.query_map(params![base, target, date], row_to_rate)?;
    rows.next().transpose()
}

pub fn get_all_for_date(conn: &Connection, base: &str, date: &str) -> Result<Vec<ExchangeRate>> {
    let mut stmt = conn.prepare(
        "SELECT base_currency, target_currency, rate, fetched_at, valid_date
         FROM exchange_rates
         WHERE base_currency=?1 AND valid_date=?2
         ORDER BY target_currency ASC",
    )?;
    let rows = stmt.query_map(params![base, date], row_to_rate)?;
    rows.collect()
}

pub fn upsert(conn: &Connection, rate: &ExchangeRate) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO exchange_rates
             (base_currency, target_currency, rate, fetched_at, valid_date)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            rate.base_currency,
            rate.target_currency,
            rate.rate,
            rate.fetched_at,
            rate.valid_date,
        ],
    )?;
    Ok(())
}

pub fn get_latest_rates(conn: &Connection, base: &str) -> Result<Vec<ExchangeRate>> {
    let mut stmt = conn.prepare(
        "SELECT base_currency, target_currency, rate, fetched_at, valid_date
         FROM exchange_rates
         WHERE base_currency=?1
         GROUP BY target_currency
         HAVING valid_date = MAX(valid_date)
         ORDER BY target_currency ASC",
    )?;
    let rows = stmt.query_map(params![base], row_to_rate)?;
    rows.collect()
}
