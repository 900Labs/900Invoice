use crate::db;
use crate::models::exchange_rate::{ConversionResult, ExchangeRate};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

/// Return all exchange rates for a base currency on the most recent available date.
#[tauri::command]
pub fn get_exchange_rates(
    db: State<'_, DbConn>,
    base_currency: String,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let rates = db::queries::exchange_rates::get_latest_rates(&conn, &base_currency)
        .map_err(|e| e.to_string())?;
    serde_json::to_value(rates).map_err(|e| e.to_string())
}

/// Return the most recent cached rate for a specific pair, optionally filtered by date.
#[tauri::command]
pub fn get_cached_rate(
    db: State<'_, DbConn>,
    base_currency: String,
    target_currency: String,
    date: Option<String>,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let effective_date =
        date.unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string());
    let rate = db::queries::exchange_rates::get_rate(
        &conn,
        &base_currency,
        &target_currency,
        &effective_date,
    )
    .map_err(|e| e.to_string())?;
    match rate {
        Some(r) => serde_json::to_value(r).map_err(|e| e.to_string()),
        None => Ok(serde_json::Value::Null),
    }
}

/// Convert an amount from one currency to another using cached rates.
#[tauri::command]
pub fn convert_currency(
    db: State<'_, DbConn>,
    from_currency: String,
    to_currency: String,
    amount_minor: i64,
    date: Option<String>,
) -> Result<serde_json::Value, String> {
    if from_currency == to_currency {
        let result = ConversionResult {
            from_currency: from_currency.clone(),
            to_currency: to_currency.clone(),
            from_amount_minor: amount_minor,
            to_amount_minor: amount_minor,
            rate: 1.0,
            valid_date: date.unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string()),
        };
        return serde_json::to_value(result).map_err(|e| e.to_string());
    }

    let conn = db.lock().map_err(|e| e.to_string())?;
    let effective_date =
        date.unwrap_or_else(|| chrono::Local::now().format("%Y-%m-%d").to_string());

    let rate_row =
        db::queries::exchange_rates::get_rate(&conn, &from_currency, &to_currency, &effective_date)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| {
                format!(
                    "No exchange rate found for {}/{} on or before {}",
                    from_currency, to_currency, effective_date
                )
            })?;

    // Convert: to_amount = from_amount * rate  (both in minor units, rate is per major unit)
    let to_amount = (amount_minor as f64 * rate_row.rate).round() as i64;

    let result = ConversionResult {
        from_currency,
        to_currency,
        from_amount_minor: amount_minor,
        to_amount_minor: to_amount,
        rate: rate_row.rate,
        valid_date: rate_row.valid_date,
    };
    serde_json::to_value(result).map_err(|e| e.to_string())
}

/// Upsert a batch of exchange rates (e.g. from an external API call on the frontend).
#[tauri::command]
pub fn upsert_exchange_rates(
    db: State<'_, DbConn>,
    rates: Vec<ExchangeRate>,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    for rate in &rates {
        db::queries::exchange_rates::upsert(&conn, rate).map_err(|e| e.to_string())?;
    }
    Ok(())
}
