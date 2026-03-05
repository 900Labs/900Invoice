use crate::db;
use crate::models::business::UpdateBusinessProfile;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

#[tauri::command]
pub fn get_business_profile(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let profile = db::queries::business::get(&conn).map_err(|e| e.to_string())?;
    match profile {
        Some(p) => serde_json::to_value(p).map_err(|e| e.to_string()),
        None => Ok(serde_json::Value::Null),
    }
}

#[tauri::command]
pub fn update_business_profile(
    db: State<'_, DbConn>,
    update: UpdateBusinessProfile,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let profile = db::queries::business::upsert(&conn, &update).map_err(|e| e.to_string())?;
    serde_json::to_value(profile).map_err(|e| e.to_string())
}
