use crate::db;
use crate::models::recurring::{CreateRecurring, UpdateRecurring};
use crate::services::recurring_scheduler;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

#[tauri::command]
pub fn list_recurring(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let recurrings = db::queries::recurring::list_all(&conn).map_err(|e| e.to_string())?;
    serde_json::to_value(recurrings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_recurring(
    db: State<'_, DbConn>,
    recurring: CreateRecurring,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let created = db::queries::recurring::insert(&conn, &recurring).map_err(|e| e.to_string())?;
    serde_json::to_value(created).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_recurring(
    db: State<'_, DbConn>,
    id: String,
    update: UpdateRecurring,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let updated = db::queries::recurring::update(&conn, &id, &update).map_err(|e| e.to_string())?;
    serde_json::to_value(updated).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_recurring(db: State<'_, DbConn>, id: String) -> Result<(), String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    db::queries::recurring::delete(&conn, &id).map_err(|e| e.to_string())
}

/// Generate actual invoices for all recurring schedules that are due today.
/// Returns the list of newly created invoices.
#[tauri::command]
pub fn generate_due_recurring(db: State<'_, DbConn>) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let generated_ids = recurring_scheduler::process_all_due(&conn)?;
    let mut generated = Vec::new();

    for invoice_id in generated_ids {
        if let Some(invoice) = db::queries::invoices::get_with_details(&conn, &invoice_id)
            .map_err(|e| e.to_string())?
        {
            generated.push(invoice);
        }
    }

    serde_json::to_value(generated).map_err(|e| e.to_string())
}
