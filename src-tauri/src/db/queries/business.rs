use crate::models::business::{BusinessProfile, UpdateBusinessProfile};
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

fn row_to_profile(row: &rusqlite::Row<'_>) -> Result<BusinessProfile> {
    Ok(BusinessProfile {
        id: row.get(0)?,
        name: row.get(1)?,
        address: row.get(2)?,
        city: row.get(3)?,
        country: row.get(4)?,
        country_code: row.get(5)?,
        phone: row.get(6)?,
        email: row.get(7)?,
        website: row.get(8)?,
        tax_id: row.get(9)?,
        logo_path: row.get(10)?,
        default_currency: row.get(11)?,
        default_payment_terms_days: row.get(12)?,
        bank_name: row.get(13)?,
        bank_account_number: row.get(14)?,
        bank_routing_number: row.get(15)?,
        mobile_money_number: row.get(16)?,
        mobile_money_provider: row.get(17)?,
        created_at: row.get(18)?,
        updated_at: row.get(19)?,
    })
}

pub fn get(conn: &Connection) -> Result<Option<BusinessProfile>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, address, city, country, country_code, phone, email, website,
                tax_id, logo_path, default_currency, default_payment_terms_days,
                bank_name, bank_account_number, bank_routing_number,
                mobile_money_number, mobile_money_provider, created_at, updated_at
         FROM business_profiles LIMIT 1",
    )?;
    let mut rows = stmt.query_map([], row_to_profile)?;
    rows.next().transpose()
}

pub fn upsert(conn: &Connection, update: &UpdateBusinessProfile) -> Result<BusinessProfile> {
    // Ensure a row exists first
    let existing = get(conn)?;
    let id = match &existing {
        Some(p) => p.id.clone(),
        None => Uuid::new_v4().to_string(),
    };

    if existing.is_none() {
        conn.execute(
            "INSERT INTO business_profiles (id) VALUES (?1)",
            params![id],
        )?;
    }

    // Build partial update
    if let Some(v) = &update.name {
        conn.execute(
            "UPDATE business_profiles SET name=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &update.address {
        conn.execute(
            "UPDATE business_profiles SET address=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &update.city {
        conn.execute(
            "UPDATE business_profiles SET city=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &update.country {
        conn.execute(
            "UPDATE business_profiles SET country=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &update.country_code {
        conn.execute(
            "UPDATE business_profiles SET country_code=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &update.phone {
        conn.execute(
            "UPDATE business_profiles SET phone=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &update.email {
        conn.execute(
            "UPDATE business_profiles SET email=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &update.website {
        conn.execute(
            "UPDATE business_profiles SET website=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &update.tax_id {
        conn.execute(
            "UPDATE business_profiles SET tax_id=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    // logo_path can be set to None
    conn.execute(
        "UPDATE business_profiles SET logo_path=?1, updated_at=datetime('now') WHERE id=?2",
        params![update.logo_path, id],
    )?;
    if let Some(v) = &update.default_currency {
        conn.execute("UPDATE business_profiles SET default_currency=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &update.default_payment_terms_days {
        conn.execute("UPDATE business_profiles SET default_payment_terms_days=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &update.bank_name {
        conn.execute(
            "UPDATE business_profiles SET bank_name=?1, updated_at=datetime('now') WHERE id=?2",
            params![v, id],
        )?;
    }
    if let Some(v) = &update.bank_account_number {
        conn.execute("UPDATE business_profiles SET bank_account_number=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &update.bank_routing_number {
        conn.execute("UPDATE business_profiles SET bank_routing_number=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &update.mobile_money_number {
        conn.execute("UPDATE business_profiles SET mobile_money_number=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }
    if let Some(v) = &update.mobile_money_provider {
        conn.execute("UPDATE business_profiles SET mobile_money_provider=?1, updated_at=datetime('now') WHERE id=?2", params![v, id])?;
    }

    // Record changelog
    super::changelog::insert_entry(conn, "business_profiles", &id, "UPDATE", "{}")?;

    get(conn)?.ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)
}
