mod commands;
mod db;
mod models;
mod services;
mod sync;

use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).ok();
            let conn = db::init_database(&app_data_dir).expect("failed to init database");
            app.manage(Mutex::new(conn));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Business
            commands::business::get_business_profile,
            commands::business::update_business_profile,
            // Clients
            commands::clients::list_clients,
            commands::clients::get_client,
            commands::clients::create_client,
            commands::clients::update_client,
            commands::clients::delete_client,
            commands::clients::search_clients,
            // Invoices
            commands::invoices::list_invoices,
            commands::invoices::get_invoice,
            commands::invoices::create_invoice,
            commands::invoices::update_invoice,
            commands::invoices::delete_invoice,
            commands::invoices::finalize_invoice,
            commands::invoices::void_invoice,
            commands::invoices::duplicate_invoice,
            commands::invoices::search_invoices,
            // Line Items
            commands::line_items::add_line_item,
            commands::line_items::update_line_item,
            commands::line_items::remove_line_item,
            commands::line_items::reorder_line_items,
            // Taxes
            commands::taxes::list_tax_rates,
            commands::taxes::create_tax_rate,
            commands::taxes::update_tax_rate,
            commands::taxes::delete_tax_rate,
            commands::taxes::get_tax_rates_for_country,
            commands::taxes::calculate_invoice_taxes,
            // PDF
            commands::pdf::generate_invoice_pdf,
            commands::pdf::get_pdf_preview_data,
            // Payments
            commands::payments::list_payments,
            commands::payments::record_payment,
            commands::payments::delete_payment,
            commands::payments::get_invoice_payment_summary,
            // Recurring
            commands::recurring::list_recurring,
            commands::recurring::create_recurring,
            commands::recurring::update_recurring,
            commands::recurring::delete_recurring,
            commands::recurring::generate_due_recurring,
            // Products
            commands::products::list_products,
            commands::products::get_product,
            commands::products::create_product,
            commands::products::update_product,
            commands::products::delete_product,
            commands::products::search_products,
            // Exchange Rates
            commands::exchange_rates::get_exchange_rates,
            commands::exchange_rates::get_cached_rate,
            commands::exchange_rates::convert_currency,
            commands::exchange_rates::upsert_exchange_rates,
            // Import/Export
            commands::import_export::import_clients_csv,
            commands::import_export::export_clients_csv,
            commands::import_export::export_invoices_csv,
            commands::import_export::backup_database,
            commands::import_export::restore_database,
            // Settings
            commands::settings::get_settings,
            commands::settings::get_setting,
            commands::settings::update_setting,
            commands::settings::get_invoice_sequence,
            commands::settings::update_invoice_sequence,
            // Sync
            commands::sync::get_changelog,
            commands::sync::get_changes_since,
        ])
        .run(tauri::generate_context!())
        .expect("error while running 900Invoice");
}
