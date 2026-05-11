use rusqlite::{Connection, Result};

fn column_exists(conn: &Connection, table: &str, column: &str) -> Result<bool> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
    let columns = stmt.query_map([], |row| row.get::<_, String>(1))?;
    for name in columns {
        if name? == column {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        -- Table 1: business_profiles
        CREATE TABLE IF NOT EXISTS business_profiles (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL DEFAULT '',
            address TEXT NOT NULL DEFAULT '',
            city TEXT NOT NULL DEFAULT '',
            country TEXT NOT NULL DEFAULT '',
            country_code TEXT NOT NULL DEFAULT '',
            phone TEXT NOT NULL DEFAULT '',
            email TEXT NOT NULL DEFAULT '',
            website TEXT NOT NULL DEFAULT '',
            tax_id TEXT NOT NULL DEFAULT '',
            logo_path TEXT,
            default_currency TEXT NOT NULL DEFAULT 'USD',
            default_payment_terms_days INTEGER NOT NULL DEFAULT 30,
            bank_name TEXT NOT NULL DEFAULT '',
            bank_account_number TEXT NOT NULL DEFAULT '',
            bank_routing_number TEXT NOT NULL DEFAULT '',
            mobile_money_number TEXT NOT NULL DEFAULT '',
            mobile_money_provider TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- Table 2: clients
        CREATE TABLE IF NOT EXISTS clients (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL DEFAULT '',
            phone TEXT NOT NULL DEFAULT '',
            address TEXT NOT NULL DEFAULT '',
            city TEXT NOT NULL DEFAULT '',
            country TEXT NOT NULL DEFAULT '',
            country_code TEXT NOT NULL DEFAULT '',
            tax_id TEXT NOT NULL DEFAULT '',
            currency_code TEXT NOT NULL DEFAULT 'USD',
            payment_terms_days INTEGER NOT NULL DEFAULT 30,
            notes TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- Table 3: invoices
        CREATE TABLE IF NOT EXISTS invoices (
            id TEXT PRIMARY KEY,
            invoice_number TEXT UNIQUE,
            client_id TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'draft',
            currency_code TEXT NOT NULL DEFAULT 'USD',
            subtotal_minor INTEGER NOT NULL DEFAULT 0,
            discount_minor INTEGER NOT NULL DEFAULT 0,
            tax_amount_minor INTEGER NOT NULL DEFAULT 0,
            total_minor INTEGER NOT NULL DEFAULT 0,
            amount_paid_minor INTEGER NOT NULL DEFAULT 0,
            exchange_rate_to_usd REAL,
            exchange_rate_date TEXT,
            issue_date TEXT NOT NULL DEFAULT (date('now')),
            due_date TEXT NOT NULL,
            uses_inclusive_taxes INTEGER NOT NULL DEFAULT 0,
            notes TEXT NOT NULL DEFAULT '',
            terms TEXT NOT NULL DEFAULT '',
            footer TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            finalized_at TEXT,
            sent_at TEXT,
            paid_at TEXT,
            voided_at TEXT,
            FOREIGN KEY (client_id) REFERENCES clients(id)
        );

        -- Table 4: invoice_line_items
        CREATE TABLE IF NOT EXISTS invoice_line_items (
            id TEXT PRIMARY KEY,
            invoice_id TEXT NOT NULL,
            product_id TEXT,
            tax_rate_id TEXT,
            description TEXT NOT NULL,
            quantity INTEGER NOT NULL DEFAULT 100,
            unit_price_minor INTEGER NOT NULL DEFAULT 0,
            tax_rate_bps INTEGER NOT NULL DEFAULT 0,
            discount_bps INTEGER NOT NULL DEFAULT 0,
            line_total_minor INTEGER NOT NULL DEFAULT 0,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE,
            FOREIGN KEY (product_id) REFERENCES products(id)
        );

        -- Table 5: invoice_taxes
        CREATE TABLE IF NOT EXISTS invoice_taxes (
            id TEXT PRIMARY KEY,
            invoice_id TEXT NOT NULL,
            tax_rate_id TEXT,
            tax_name TEXT NOT NULL,
            tax_rate_bps INTEGER NOT NULL,
            tax_amount_minor INTEGER NOT NULL DEFAULT 0,
            is_withholding INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE CASCADE
        );

        -- Table 6: tax_rates
        CREATE TABLE IF NOT EXISTS tax_rates (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            display_name TEXT NOT NULL,
            rate_bps INTEGER NOT NULL,
            country_code TEXT,
            is_default INTEGER NOT NULL DEFAULT 0,
            is_withholding INTEGER NOT NULL DEFAULT 0,
            is_inclusive INTEGER NOT NULL DEFAULT 0,
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- Table 7: payments
        CREATE TABLE IF NOT EXISTS payments (
            id TEXT PRIMARY KEY,
            invoice_id TEXT NOT NULL,
            amount_minor INTEGER NOT NULL,
            currency_code TEXT NOT NULL,
            payment_method TEXT NOT NULL DEFAULT 'cash',
            payment_reference TEXT NOT NULL DEFAULT '',
            notes TEXT NOT NULL DEFAULT '',
            paid_at TEXT NOT NULL DEFAULT (datetime('now')),
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (invoice_id) REFERENCES invoices(id)
        );

        -- Table 8: recurring_invoices
        CREATE TABLE IF NOT EXISTS recurring_invoices (
            id TEXT PRIMARY KEY,
            client_id TEXT NOT NULL,
            template_invoice_id TEXT NOT NULL,
            frequency TEXT NOT NULL DEFAULT 'monthly',
            next_generation_date TEXT NOT NULL,
            end_date TEXT,
            auto_send INTEGER NOT NULL DEFAULT 0,
            last_generated TEXT,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (client_id) REFERENCES clients(id),
            FOREIGN KEY (template_invoice_id) REFERENCES invoices(id)
        );

        -- Table 9: invoice_sequences
        CREATE TABLE IF NOT EXISTS invoice_sequences (
            sequence_name TEXT PRIMARY KEY,
            prefix TEXT NOT NULL DEFAULT 'INV',
            separator TEXT NOT NULL DEFAULT '-',
            include_year INTEGER NOT NULL DEFAULT 1,
            pad_digits INTEGER NOT NULL DEFAULT 4,
            year_reset INTEGER NOT NULL DEFAULT 1,
            last_year INTEGER,
            last_month INTEGER,
            next_number INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- Table 10: exchange_rates
        CREATE TABLE IF NOT EXISTS exchange_rates (
            base_currency TEXT NOT NULL,
            target_currency TEXT NOT NULL,
            rate REAL NOT NULL,
            fetched_at TEXT NOT NULL,
            valid_date TEXT NOT NULL,
            PRIMARY KEY (base_currency, target_currency, valid_date)
        );

        -- Table 11: products
        CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            default_price_minor INTEGER NOT NULL DEFAULT 0,
            default_currency TEXT NOT NULL DEFAULT 'USD',
            default_tax_rate_id TEXT,
            default_tax_rate_bps INTEGER NOT NULL DEFAULT 0,
            unit TEXT NOT NULL DEFAULT 'unit',
            is_active INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- Table 12: changelog
        CREATE TABLE IF NOT EXISTS changelog (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            table_name TEXT NOT NULL,
            row_id TEXT NOT NULL,
            operation TEXT NOT NULL,
            payload TEXT NOT NULL DEFAULT '{}',
            timestamp TEXT NOT NULL DEFAULT (datetime('now'))
        );

        -- Table 13: settings
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL DEFAULT ''
        );
        ",
    )?;

    if !column_exists(conn, "invoice_line_items", "tax_rate_id")? {
        conn.execute(
            "ALTER TABLE invoice_line_items ADD COLUMN tax_rate_id TEXT",
            [],
        )?;
    }
    if !column_exists(conn, "products", "default_tax_rate_id")? {
        conn.execute(
            "ALTER TABLE products ADD COLUMN default_tax_rate_id TEXT",
            [],
        )?;
    }

    // Insert default data (OR IGNORE so re-running is idempotent)
    conn.execute_batch(
        "
        INSERT OR IGNORE INTO invoice_sequences (sequence_name, prefix, next_number)
            VALUES ('default', 'INV', 1);

        INSERT OR IGNORE INTO tax_rates (id, name, display_name, rate_bps, country_code, is_default, is_withholding) VALUES
            ('tax-ke-vat',    'VAT',     'VAT @ 16%',      1600, 'KE',  1, 0),
            ('tax-ke-wht',    'WHT',     'WHT @ 5%',        500, 'KE',  0, 1),
            ('tax-ng-vat',    'VAT',     'VAT @ 7.5%',      750, 'NG',  1, 0),
            ('tax-ng-wht5',   'WHT',     'WHT @ 5%',        500, 'NG',  0, 1),
            ('tax-ng-wht10',  'WHT',     'WHT @ 10%',      1000, 'NG',  0, 1),
            ('tax-za-vat',    'VAT',     'VAT @ 15%',      1500, 'ZA',  1, 0),
            ('tax-in-gst18',  'GST',     'GST @ 18%',      1800, 'IN',  1, 0),
            ('tax-in-gst5',   'GST',     'GST @ 5%',        500, 'IN',  0, 0),
            ('tax-gh-vat',    'VAT',     'VAT @ 15%',      1500, 'GH',  1, 0),
            ('tax-gh-nhil',   'NHIL',    'NHIL @ 2.5%',     250, 'GH',  0, 0),
            ('tax-gh-getfund','GETFund', 'GETFund @ 2.5%',  250, 'GH',  0, 0),
            ('tax-tz-vat',    'VAT',     'VAT @ 18%',      1800, 'TZ',  1, 0),
            ('tax-ug-vat',    'VAT',     'VAT @ 18%',      1800, 'UG',  1, 0),
            ('tax-xof-vat',   'VAT',     'VAT @ 18%',      1800, 'SN',  1, 0),
            ('tax-none',      'No Tax',  'No Tax',            0, NULL,  0, 0);

        UPDATE tax_rates
           SET is_withholding = 1
         WHERE id IN ('tax-ke-wht', 'tax-ng-wht5', 'tax-ng-wht10');

        INSERT OR IGNORE INTO settings (key, value) VALUES
            ('locale',            '\"en\"'),
            ('theme',             '\"light\"'),
            ('date_format',       '\"YYYY-MM-DD\"'),
            ('currency_position', '\"before\"'),
            ('paper_size',        '\"a4\"');
        ",
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run_migrations;
    use rusqlite::Connection;

    #[test]
    fn migrations_seed_withholding_rates_and_tax_identity_columns() {
        let conn = Connection::open_in_memory().expect("in-memory db");
        run_migrations(&conn).expect("migrations");

        let tax_rate_id_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('invoice_line_items') WHERE name='tax_rate_id'",
                [],
                |row| row.get(0),
            )
            .expect("tax_rate_id column");
        assert_eq!(tax_rate_id_exists, 1);

        let product_tax_rate_id_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM pragma_table_info('products') WHERE name='default_tax_rate_id'",
                [],
                |row| row.get(0),
            )
            .expect("product default_tax_rate_id column");
        assert_eq!(product_tax_rate_id_exists, 1);

        let withholding_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM tax_rates
                 WHERE id IN ('tax-ke-wht', 'tax-ng-wht5', 'tax-ng-wht10')
                   AND is_withholding=1",
                [],
                |row| row.get(0),
            )
            .expect("withholding tax count");
        assert_eq!(withholding_count, 3);
    }
}
