use crate::db;
use crate::models::client::{CreateClient, UpdateClient};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

#[tauri::command]
pub fn list_clients(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let clients = db::queries::clients::list_all(&conn).map_err(|e| e.to_string())?;
    serde_json::to_value(clients).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_client(db: State<'_, DbConn>, id: String) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let client = db::queries::clients::get_by_id(&conn, &id).map_err(|e| e.to_string())?;
    match client {
        Some(c) => serde_json::to_value(c).map_err(|e| e.to_string()),
        None => Err(format!("Client not found: {}", id)),
    }
}

#[tauri::command]
pub fn create_client(
    db: State<'_, DbConn>,
    client: CreateClient,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let created = db::queries::clients::insert(&conn, &client).map_err(|e| e.to_string())?;
    serde_json::to_value(created).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_client(
    db: State<'_, DbConn>,
    id: String,
    update: UpdateClient,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let updated = db::queries::clients::update(&conn, &id, &update).map_err(|e| e.to_string())?;
    serde_json::to_value(updated).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_client(db: State<'_, DbConn>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    db::queries::clients::delete(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_clients(db: State<'_, DbConn>, query: String) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let results = db::queries::clients::search(&conn, &query).map_err(|e| e.to_string())?;
    serde_json::to_value(results).map_err(|e| e.to_string())
}
