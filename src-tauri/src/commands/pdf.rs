use crate::db;
use crate::services::pdf_engine;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

type DbConn = Mutex<Connection>;

/// Returns a base64-encoded HTML string (placeholder for full PDF generation).
/// In v1.0 the HTML is base64-encoded and returned as a "PDF" so the frontend
/// can open it in a webview or print dialog.
#[tauri::command]
pub fn generate_invoice_pdf(
    db: State<'_, DbConn>,
    invoice_id: String,
) -> Result<String, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let invoice = db::queries::invoices::get_with_details(&conn, &invoice_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", invoice_id))?;

    let business = db::queries::business::get(&conn)
        .map_err(|e| e.to_string())?
        .unwrap_or_else(default_business);

    let html = pdf_engine::generate_invoice_html(&invoice, &business);

    // Base64-encode the HTML for transport
    use std::io::Write;
    let mut buf = Vec::new();
    buf.write_all(html.as_bytes()).map_err(|e| e.to_string())?;
    Ok(base64_encode(&buf))
}

/// Returns structured JSON data suitable for the frontend to render a PDF preview.
#[tauri::command]
pub fn get_pdf_preview_data(
    db: State<'_, DbConn>,
    invoice_id: String,
) -> Result<serde_json::Value, String> {
    let conn = db.lock().map_err(|e| e.to_string())?;

    let invoice = db::queries::invoices::get_with_details(&conn, &invoice_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Invoice not found: {}", invoice_id))?;

    let business = db::queries::business::get(&conn)
        .map_err(|e| e.to_string())?
        .unwrap_or_else(default_business);

    let html = pdf_engine::generate_invoice_html(&invoice, &business);

    serde_json::to_value(serde_json::json!({
        "invoice": invoice,
        "business": business,
        "html": html,
    }))
    .map_err(|e| e.to_string())
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
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let combined = (b0 << 16) | (b1 << 8) | b2;
        out.push(CHARS[((combined >> 18) & 0x3f) as usize] as char);
        out.push(CHARS[((combined >> 12) & 0x3f) as usize] as char);
        out.push(if chunk.len() > 1 { CHARS[((combined >> 6) & 0x3f) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { CHARS[(combined & 0x3f) as usize] as char } else { '=' });
    }
    out
}
