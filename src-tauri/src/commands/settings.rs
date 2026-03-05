use crate::db;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

#[tauri::command]
pub fn get_settings(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let map = db::queries::settings::get_all(&conn).map_err(|e| e.to_string())?;
    serde_json::to_value(map).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_setting(db: State<'_, DbConn>, key: String) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let value = db::queries::settings::get_by_key(&conn, &key).map_err(|e| e.to_string())?;
    match value {
        Some(v) => {
            // Values are stored as JSON strings (e.g. `"\"en\""`)
            let parsed: serde_json::Value =
                serde_json::from_str(&v).unwrap_or(serde_json::Value::String(v));
            Ok(parsed)
        }
        None => Ok(serde_json::Value::Null),
    }
}

#[tauri::command]
pub fn update_setting(
    db: State<'_, DbConn>,
    key: String,
    value: serde_json::Value,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let serialized = serde_json::to_string(&value).map_err(|e| e.to_string())?;
    db::queries::settings::upsert(&conn, &key, &serialized).map_err(|e| e.to_string())
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceSequenceInfo {
    pub prefix: String,
    pub separator: String,
    pub include_year: bool,
    pub pad_digits: i32,
    pub year_reset: bool,
    pub next_number: i32,
}

#[tauri::command]
pub fn get_invoice_sequence(db: State<'_, DbConn>) -> Result<InvoiceSequenceInfo, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT prefix, separator, include_year, pad_digits, year_reset, next_number \
             FROM invoice_sequences WHERE sequence_name = 'default'",
        )
        .map_err(|e| e.to_string())?;
    stmt.query_row([], |row| {
        Ok(InvoiceSequenceInfo {
            prefix: row.get(0)?,
            separator: row.get(1)?,
            include_year: row.get::<_, i32>(2)? != 0,
            pad_digits: row.get(3)?,
            year_reset: row.get::<_, i32>(4)? != 0,
            next_number: row.get(5)?,
        })
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_invoice_sequence(
    db: State<'_, DbConn>,
    sequence: InvoiceSequenceInfo,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE invoice_sequences SET prefix=?1, separator=?2, include_year=?3, \
         pad_digits=?4, year_reset=?5, next_number=?6 WHERE sequence_name='default'",
        rusqlite::params![
            sequence.prefix,
            sequence.separator,
            sequence.include_year as i32,
            sequence.pad_digits,
            sequence.year_reset as i32,
            sequence.next_number,
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
