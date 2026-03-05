use crate::commands::invoices::recalculate_invoice_totals;
use crate::db;
use crate::models::line_item::{CreateLineItem, UpdateLineItem};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

#[tauri::command]
pub fn add_line_item(
    db: State<'_, DbConn>,
    line_item: CreateLineItem,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let invoice_id = line_item.invoice_id.clone();
    let created =
        db::queries::line_items::insert(&conn, &line_item).map_err(|e| e.to_string())?;
    recalculate_invoice_totals(&conn, &invoice_id)?;
    serde_json::to_value(created).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_line_item(
    db: State<'_, DbConn>,
    id: String,
    update: UpdateLineItem,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let updated =
        db::queries::line_items::update(&conn, &id, &update).map_err(|e| e.to_string())?;
    recalculate_invoice_totals(&conn, &updated.invoice_id)?;
    serde_json::to_value(updated).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn remove_line_item(db: State<'_, DbConn>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    // Get invoice_id before deletion for recalculation
    let item = db::queries::line_items::get_by_id(&conn, &id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Line item not found: {}", id))?;
    let invoice_id = item.invoice_id.clone();
    db::queries::line_items::delete(&conn, &id).map_err(|e| e.to_string())?;
    recalculate_invoice_totals(&conn, &invoice_id)?;
    Ok(())
}

#[tauri::command]
pub fn reorder_line_items(
    db: State<'_, DbConn>,
    ordered_ids: Vec<String>,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    db::queries::line_items::reorder(&conn, &ordered_ids).map_err(|e| e.to_string())
}
