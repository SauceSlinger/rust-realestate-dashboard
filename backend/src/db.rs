use sqlx::{sqlite::SqlitePool, Row};
use crate::models::*;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // Use sqlite: URL with file creation enabled
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./realestate.db?mode=rwc".to_string());
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Create properties table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS properties (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            address TEXT NOT NULL,
            city TEXT NOT NULL,
            state TEXT NOT NULL,
            zip_code TEXT NOT NULL,
            property_type TEXT NOT NULL,
            bedrooms INTEGER NOT NULL,
            bathrooms REAL NOT NULL,
            square_feet INTEGER NOT NULL,
            purchase_price REAL NOT NULL,
            current_value REAL NOT NULL,
            status TEXT NOT NULL,
            notes TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Create reminders table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS reminders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            property_id INTEGER,
            title TEXT NOT NULL,
            description TEXT,
            due_date DATE NOT NULL,
            completed BOOLEAN DEFAULT 0,
            reminder_type TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (property_id) REFERENCES properties(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Create market_data table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS market_data (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            location TEXT NOT NULL,
            median_price REAL NOT NULL,
            average_price_per_sqft REAL NOT NULL,
            inventory_count INTEGER NOT NULL,
            days_on_market INTEGER NOT NULL,
            data_date DATE NOT NULL,
            source TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Create tenants table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tenants (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            property_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone TEXT NOT NULL,
            lease_start DATE NOT NULL,
            lease_end DATE NOT NULL,
            monthly_rent REAL NOT NULL,
            deposit REAL NOT NULL,
            status TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (property_id) REFERENCES properties(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

// Property CRUD operations
pub async fn create_property(pool: &SqlitePool, property: CreateProperty) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO properties (address, city, state, zip_code, property_type, bedrooms, bathrooms, square_feet, purchase_price, current_value, status, notes)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&property.address)
    .bind(&property.city)
    .bind(&property.state)
    .bind(&property.zip_code)
    .bind(&property.property_type)
    .bind(property.bedrooms)
    .bind(property.bathrooms)
    .bind(property.square_feet)
    .bind(property.purchase_price)
    .bind(property.current_value)
    .bind(&property.status)
    .bind(&property.notes)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_properties(pool: &SqlitePool) -> Result<Vec<Property>, sqlx::Error> {
    let properties = sqlx::query_as::<_, Property>(
        r#"SELECT id, address, city, state, zip_code, property_type, bedrooms, bathrooms, square_feet, purchase_price, current_value, status, notes, created_at, updated_at FROM properties"#
    )
    .fetch_all(pool)
    .await?;

    Ok(properties)
}

pub async fn get_property(pool: &SqlitePool, id: i64) -> Result<Option<Property>, sqlx::Error> {
    let property = sqlx::query_as::<_, Property>(
        r#"SELECT id, address, city, state, zip_code, property_type, bedrooms, bathrooms, square_feet, purchase_price, current_value, status, notes, created_at, updated_at FROM properties WHERE id = ?"#
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(property)
}

pub async fn update_property(pool: &SqlitePool, id: i64, property: UpdateProperty) -> Result<bool, sqlx::Error> {
    let existing = get_property(pool, id).await?;
    if existing.is_none() {
        return Ok(false);
    }
    
    let existing = existing.unwrap();
    
    let result = sqlx::query(
        r#"
        UPDATE properties 
        SET address = ?, city = ?, state = ?, zip_code = ?, property_type = ?, 
            bedrooms = ?, bathrooms = ?, square_feet = ?, purchase_price = ?, 
            current_value = ?, status = ?, notes = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        "#,
    )
    .bind(property.address.unwrap_or(existing.address))
    .bind(property.city.unwrap_or(existing.city))
    .bind(property.state.unwrap_or(existing.state))
    .bind(property.zip_code.unwrap_or(existing.zip_code))
    .bind(property.property_type.unwrap_or(existing.property_type))
    .bind(property.bedrooms.unwrap_or(existing.bedrooms))
    .bind(property.bathrooms.unwrap_or(existing.bathrooms))
    .bind(property.square_feet.unwrap_or(existing.square_feet))
    .bind(property.purchase_price.unwrap_or(existing.purchase_price))
    .bind(property.current_value.unwrap_or(existing.current_value))
    .bind(property.status.unwrap_or(existing.status))
    .bind(property.notes.or(existing.notes))
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn delete_property(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM properties WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

// Reminder CRUD operations
pub async fn create_reminder(pool: &SqlitePool, reminder: CreateReminder) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO reminders (property_id, title, description, due_date, reminder_type)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(reminder.property_id)
    .bind(&reminder.title)
    .bind(&reminder.description)
    .bind(&reminder.due_date)
    .bind(&reminder.reminder_type)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_reminders(pool: &SqlitePool) -> Result<Vec<Reminder>, sqlx::Error> {
    let reminders = sqlx::query_as::<_, Reminder>(
        r#"SELECT id, property_id, title, description, due_date, completed, reminder_type, created_at FROM reminders ORDER BY due_date ASC"#
    )
    .fetch_all(pool)
    .await?;

    Ok(reminders)
}

pub async fn update_reminder(pool: &SqlitePool, id: i64, reminder: UpdateReminder) -> Result<bool, sqlx::Error> {
    let existing = sqlx::query_as::<_, Reminder>(
        "SELECT id, property_id, title, description, due_date, completed, reminder_type, created_at FROM reminders WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    
    if existing.is_none() {
        return Ok(false);
    }
    
    let existing = existing.unwrap();
    
    let result = sqlx::query(
        r#"
        UPDATE reminders 
        SET property_id = ?, title = ?, description = ?, due_date = ?, completed = ?, reminder_type = ?
        WHERE id = ?
        "#,
    )
    .bind(reminder.property_id.or(existing.property_id))
    .bind(reminder.title.unwrap_or(existing.title))
    .bind(reminder.description.or(existing.description))
    .bind(reminder.due_date.unwrap_or(existing.due_date))
    .bind(reminder.completed.unwrap_or(existing.completed))
    .bind(reminder.reminder_type.unwrap_or(existing.reminder_type))
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn delete_reminder(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM reminders WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

// Market Data operations
pub async fn create_market_data(pool: &SqlitePool, data: CreateMarketData) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO market_data (location, median_price, average_price_per_sqft, inventory_count, days_on_market, data_date, source)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&data.location)
    .bind(data.median_price)
    .bind(data.average_price_per_sqft)
    .bind(data.inventory_count)
    .bind(data.days_on_market)
    .bind(&data.data_date)
    .bind(&data.source)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_market_data(pool: &SqlitePool, location: Option<String>) -> Result<Vec<MarketData>, sqlx::Error> {
    let data = if let Some(loc) = location {
        sqlx::query_as::<_, MarketData>(
            r#"SELECT id, location, median_price, average_price_per_sqft, inventory_count, days_on_market, data_date, source, created_at FROM market_data WHERE location = ? ORDER BY data_date DESC"#
        )
        .bind(loc)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, MarketData>(
            r#"SELECT id, location, median_price, average_price_per_sqft, inventory_count, days_on_market, data_date, source, created_at FROM market_data ORDER BY data_date DESC"#
        )
        .fetch_all(pool)
        .await?
    };

    Ok(data)
}

// Tenant CRUD operations
pub async fn create_tenant(pool: &SqlitePool, tenant: CreateTenant) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO tenants (property_id, name, email, phone, lease_start, lease_end, monthly_rent, deposit, status)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(tenant.property_id)
    .bind(&tenant.name)
    .bind(&tenant.email)
    .bind(&tenant.phone)
    .bind(&tenant.lease_start)
    .bind(&tenant.lease_end)
    .bind(tenant.monthly_rent)
    .bind(tenant.deposit)
    .bind(&tenant.status)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_tenants(pool: &SqlitePool, property_id: Option<i64>) -> Result<Vec<Tenant>, sqlx::Error> {
    let tenants = if let Some(pid) = property_id {
        sqlx::query_as::<_, Tenant>(
            r#"SELECT id, property_id, name, email, phone, lease_start, lease_end, monthly_rent, deposit, status, created_at FROM tenants WHERE property_id = ?"#
        )
        .bind(pid)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, Tenant>(
            r#"SELECT id, property_id, name, email, phone, lease_start, lease_end, monthly_rent, deposit, status, created_at FROM tenants"#
        )
        .fetch_all(pool)
        .await?
    };

    Ok(tenants)
}

pub async fn update_tenant(pool: &SqlitePool, id: i64, tenant: UpdateTenant) -> Result<bool, sqlx::Error> {
    let existing = sqlx::query_as::<_, Tenant>(
        "SELECT id, property_id, name, email, phone, lease_start, lease_end, monthly_rent, deposit, status, created_at FROM tenants WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    
    if existing.is_none() {
        return Ok(false);
    }
    
    let existing = existing.unwrap();
    
    let result = sqlx::query(
        r#"
        UPDATE tenants 
        SET property_id = ?, name = ?, email = ?, phone = ?, lease_start = ?, lease_end = ?, monthly_rent = ?, deposit = ?, status = ?
        WHERE id = ?
        "#,
    )
    .bind(tenant.property_id.unwrap_or(existing.property_id))
    .bind(tenant.name.unwrap_or(existing.name))
    .bind(tenant.email.unwrap_or(existing.email))
    .bind(tenant.phone.unwrap_or(existing.phone))
    .bind(tenant.lease_start.unwrap_or(existing.lease_start))
    .bind(tenant.lease_end.unwrap_or(existing.lease_end))
    .bind(tenant.monthly_rent.unwrap_or(existing.monthly_rent))
    .bind(tenant.deposit.unwrap_or(existing.deposit))
    .bind(tenant.status.unwrap_or(existing.status))
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn delete_tenant(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM tenants WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

// Implement FromRow for models
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for Property {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Property {
            id: row.try_get("id")?,
            address: row.try_get("address")?,
            city: row.try_get("city")?,
            state: row.try_get("state")?,
            zip_code: row.try_get("zip_code")?,
            property_type: row.try_get("property_type")?,
            bedrooms: row.try_get("bedrooms")?,
            bathrooms: row.try_get("bathrooms")?,
            square_feet: row.try_get("square_feet")?,
            purchase_price: row.try_get("purchase_price")?,
            current_value: row.try_get("current_value")?,
            status: row.try_get("status")?,
            notes: row.try_get("notes")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for Reminder {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Reminder {
            id: row.try_get("id")?,
            property_id: row.try_get("property_id")?,
            title: row.try_get("title")?,
            description: row.try_get("description")?,
            due_date: row.try_get("due_date")?,
            completed: row.try_get("completed")?,
            reminder_type: row.try_get("reminder_type")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for MarketData {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(MarketData {
            id: row.try_get("id")?,
            location: row.try_get("location")?,
            median_price: row.try_get("median_price")?,
            average_price_per_sqft: row.try_get("average_price_per_sqft")?,
            inventory_count: row.try_get("inventory_count")?,
            days_on_market: row.try_get("days_on_market")?,
            data_date: row.try_get("data_date")?,
            source: row.try_get("source")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for Tenant {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Tenant {
            id: row.try_get("id")?,
            property_id: row.try_get("property_id")?,
            name: row.try_get("name")?,
            email: row.try_get("email")?,
            phone: row.try_get("phone")?,
            lease_start: row.try_get("lease_start")?,
            lease_end: row.try_get("lease_end")?,
            monthly_rent: row.try_get("monthly_rent")?,
            deposit: row.try_get("deposit")?,
            status: row.try_get("status")?,
            created_at: row.try_get("created_at")?,
        })
    }
}
