use crate::db;
use crate::services::pdf_engine;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

/// Returns a base64-encoded PDF document for native file export.
#[tauri::command]
pub fn generate_invoice_pdf(db: State<'_, DbConn>, invoice_id: String) -> Result<String, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let settings = load_pdf_render_settings(&conn)?;

    let invoice = db::queries::invoices::get_with_details(&conn, &invoice_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", invoice_id))?;

    let business = db::queries::business::get(&conn)
        .map_err(|e| e.to_string())?
        .unwrap_or_else(default_business);

    let pdf = pdf_engine::generate_invoice_pdf_bytes(
        &invoice,
        &business,
        &settings.paper_size,
        &settings.locale,
        &settings.date_format,
    );
    Ok(base64_encode(&pdf))
}

/// Returns structured JSON data suitable for the frontend to render a PDF preview.
#[tauri::command]
pub fn get_pdf_preview_data(
    db: State<'_, DbConn>,
    invoice_id: String,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;
    let settings = load_pdf_render_settings(&conn)?;

    let invoice = db::queries::invoices::get_with_details(&conn, &invoice_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", invoice_id))?;

    let business = db::queries::business::get(&conn)
        .map_err(|e| e.to_string())?
        .unwrap_or_else(default_business);

    let html = pdf_engine::generate_invoice_html(
        &invoice,
        &business,
        &settings.paper_size,
        &settings.locale,
        &settings.date_format,
    );
    let preview = pdf_engine::get_preview_data_with_locale(
        &invoice,
        &business,
        &settings.locale,
        &settings.date_format,
    );

    serde_json::to_value(serde_json::json!({
        "invoice": invoice,
        "business": business,
        "html": html,
        "preview": preview,
    }))
    .map_err(|e| e.to_string())
}

struct PdfRenderSettings {
    paper_size: String,
    locale: String,
    date_format: String,
}

fn load_pdf_render_settings(conn: &Connection) -> Result<PdfRenderSettings, String> {
    Ok(PdfRenderSettings {
        paper_size: normalize_paper_size(&read_setting_string(conn, "paper_size", "a4")?),
        locale: normalize_locale(&read_setting_string(conn, "locale", "en")?),
        date_format: normalize_date_format(&read_setting_string(
            conn,
            "date_format",
            "YYYY-MM-DD",
        )?),
    })
}

fn read_setting_string(conn: &Connection, key: &str, default: &str) -> Result<String, String> {
    let raw = db::queries::settings::get_by_key(conn, key).map_err(|e| e.to_string())?;
    Ok(raw
        .and_then(|value| parse_setting_string(&value))
        .unwrap_or_else(|| default.to_string()))
}

fn parse_setting_string(raw: &str) -> Option<String> {
    match serde_json::from_str::<serde_json::Value>(raw) {
        Ok(serde_json::Value::String(value)) => Some(value),
        Ok(value) if value.is_null() => None,
        Ok(value) => Some(value.to_string()),
        Err(_) if raw.trim().is_empty() => None,
        Err(_) => Some(raw.to_string()),
    }
}

fn normalize_paper_size(value: &str) -> String {
    if value.trim().eq_ignore_ascii_case("letter") {
        "letter".to_string()
    } else {
        "a4".to_string()
    }
}

fn normalize_locale(value: &str) -> String {
    match value
        .trim()
        .to_lowercase()
        .split('-')
        .next()
        .unwrap_or("en")
    {
        "fr" => "fr",
        "es" => "es",
        "ar" => "ar",
        "sw" => "sw",
        "hi" => "hi",
        _ => "en",
    }
    .to_string()
}

fn normalize_date_format(value: &str) -> String {
    match value.trim() {
        "DD/MM/YYYY" => "DD/MM/YYYY",
        "MM/DD/YYYY" => "MM/DD/YYYY",
        "DD.MM.YYYY" => "DD.MM.YYYY",
        "MMM D, YYYY" => "MMM D, YYYY",
        _ => "YYYY-MM-DD",
    }
    .to_string()
}

fn default_business() -> crate::models::business::BusinessProfile {
    crate::models::business::BusinessProfile {
        id: String::new(),
        name: String::new(),
        address: String::new(),
        city: String::new(),
        country: String::new(),
        country_code: String::new(),
        phone: String::new(),
        email: String::new(),
        website: String::new(),
        tax_id: String::new(),
        logo_path: None,
        default_currency: "USD".to_string(),
        default_payment_terms_days: 30,
        bank_name: String::new(),
        bank_account_number: String::new(),
        bank_routing_number: String::new(),
        mobile_money_number: String::new(),
        mobile_money_provider: String::new(),
        created_at: String::new(),
        updated_at: String::new(),
    }
}

/// Minimal base64 encoder (no external crate required).
fn base64_encode(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let combined = (b0 << 16) | (b1 << 8) | b2;
        out.push(CHARS[((combined >> 18) & 0x3f) as usize] as char);
        out.push(CHARS[((combined >> 12) & 0x3f) as usize] as char);
        out.push(if chunk.len() > 1 {
            CHARS[((combined >> 6) & 0x3f) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            CHARS[(combined & 0x3f) as usize] as char
        } else {
            '='
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params;

    fn settings_conn() -> Connection {
        let conn = Connection::open_in_memory().expect("open settings db");
        conn.execute(
            "CREATE TABLE settings (key TEXT PRIMARY KEY, value TEXT NOT NULL)",
            [],
        )
        .expect("create settings");
        conn
    }

    fn insert_setting(conn: &Connection, key: &str, value: &str) {
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )
        .expect("insert setting");
    }

    #[test]
    fn load_pdf_render_settings_reads_json_strings() {
        let conn = settings_conn();
        insert_setting(&conn, "paper_size", "\"Letter\"");
        insert_setting(&conn, "locale", "\"fr\"");
        insert_setting(&conn, "date_format", "\"DD/MM/YYYY\"");

        let settings = load_pdf_render_settings(&conn).expect("load pdf settings");

        assert_eq!(settings.paper_size, "letter");
        assert_eq!(settings.locale, "fr");
        assert_eq!(settings.date_format, "DD/MM/YYYY");
    }

    #[test]
    fn load_pdf_render_settings_defaults_unsupported_values() {
        let conn = settings_conn();
        insert_setting(&conn, "paper_size", "\"Legal\"");
        insert_setting(&conn, "locale", "\"pt-BR\"");
        insert_setting(&conn, "date_format", "\"YYYY/MM/DD\"");

        let settings = load_pdf_render_settings(&conn).expect("load pdf settings");

        assert_eq!(settings.paper_size, "a4");
        assert_eq!(settings.locale, "en");
        assert_eq!(settings.date_format, "YYYY-MM-DD");
    }
}
