use crate::db;
use crate::models::product::{CreateProduct, UpdateProduct};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

#[tauri::command]
pub fn list_products(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let products = db::queries::products::list_all(&conn).map_err(|e| e.to_string())?;
    serde_json::to_value(products).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_product(db: State<'_, DbConn>, id: String) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let product = db::queries::products::get_by_id(&conn, &id).map_err(|e| e.to_string())?;
    match product {
        Some(p) => serde_json::to_value(p).map_err(|e| e.to_string()),
        None => Err(format!("Product not found: {}", id)),
    }
}

#[tauri::command]
pub fn create_product(
    db: State<'_, DbConn>,
    product: CreateProduct,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let created = db::queries::products::insert(&conn, &product).map_err(|e| e.to_string())?;
    serde_json::to_value(created).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_product(
    db: State<'_, DbConn>,
    id: String,
    update: UpdateProduct,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let updated =
        db::queries::products::update(&conn, &id, &update).map_err(|e| e.to_string())?;
    serde_json::to_value(updated).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_product(db: State<'_, DbConn>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    db::queries::products::delete(&conn, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn search_products(
    db: State<'_, DbConn>,
    query: String,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let results = db::queries::products::search(&conn, &query).map_err(|e| e.to_string())?;
    serde_json::to_value(results).map_err(|e| e.to_string())
}
