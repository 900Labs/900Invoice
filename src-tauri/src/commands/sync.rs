use crate::db;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

#[tauri::command]
pub fn get_changelog(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let entries = db::queries::changelog::get_all(&conn).map_err(|e| e.to_string())?;
    serde_json::to_value(entries).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_changes_since(
    db: State<'_, DbConn>,
    since: String,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let entries = db::queries::changelog::get_since(&conn, &since).map_err(|e| e.to_string())?;
    serde_json::to_value(entries).map_err(|e| e.to_string())
}
