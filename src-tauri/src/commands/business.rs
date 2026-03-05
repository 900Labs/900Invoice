use crate::db;
use crate::models::business::UpdateBusinessProfile;
use rusqlite::Connection;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{Manager, State};
use uuid::Uuid;

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
    app: tauri::AppHandle,
    db: State<'_, DbConn>,
    mut update: UpdateBusinessProfile,
) -> Result<serde_json::Value, String> {
    update.logo_path = normalize_and_store_logo(&app, update.logo_path)?;
    let conn = db.lock().map_err(|e| e.to_string())?;
    let profile = db::queries::business::upsert(&conn, &update).map_err(|e| e.to_string())?;
    serde_json::to_value(profile).map_err(|e| e.to_string())
}

fn normalize_and_store_logo(
    app: &tauri::AppHandle,
    logo_path: Option<String>,
) -> Result<Option<String>, String> {
    let Some(raw) = logo_path else {
        return Ok(None);
    };

    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let src = PathBuf::from(trimmed);
    if !src.exists() {
        return Err("Logo path does not exist".to_string());
    }
    if !src.is_file() {
        return Err("Logo path must point to a file".to_string());
    }

    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .ok_or_else(|| "Logo file must have an extension".to_string())?;

    if !matches!(
        ext.as_str(),
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp"
    ) {
        return Err("Unsupported logo file type".to_string());
    }

    let metadata = std::fs::metadata(&src).map_err(|e| e.to_string())?;
    const MAX_LOGO_BYTES: u64 = 2 * 1024 * 1024;
    if metadata.len() > MAX_LOGO_BYTES {
        return Err("Logo file is too large (max 2 MB)".to_string());
    }

    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to resolve app data dir: {e}"))?;
    let logos_dir = app_data_dir.join("logos");
    std::fs::create_dir_all(&logos_dir).map_err(|e| e.to_string())?;

    let filename = format!("{}.{}", Uuid::new_v4(), ext);
    let dest = logos_dir.join(filename);

    std::fs::copy(&src, &dest).map_err(|e| e.to_string())?;

    // Store canonical path to avoid traversal on reads.
    let canonical = std::fs::canonicalize(&dest).map_err(|e| e.to_string())?;
    if !is_within(&canonical, &logos_dir) {
        return Err("Resolved logo path is outside allowed directory".to_string());
    }

    Ok(Some(canonical.to_string_lossy().to_string()))
}

fn is_within(path: &Path, root: &Path) -> bool {
    let Ok(root_canon) = std::fs::canonicalize(root) else {
        return false;
    };
    path.starts_with(root_canon)
}
