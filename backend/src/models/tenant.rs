use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: i64,
    pub property_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub lease_start: DateTime<Utc>,
    pub lease_end: DateTime<Utc>,
    pub monthly_rent: f64,
    pub deposit_amount: Option<f64>,
    pub status: String, // active, past, pending
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTenant {
    pub property_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub lease_start: DateTime<Utc>,
    pub lease_end: DateTime<Utc>,
    pub monthly_rent: f64,
    pub deposit_amount: Option<f64>,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTenant {
    pub property_id: Option<i64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub lease_start: Option<DateTime<Utc>>,
    pub lease_end: Option<DateTime<Utc>>,
    pub monthly_rent: Option<f64>,
    pub deposit_amount: Option<f64>,
    pub status: Option<String>,
    pub notes: Option<String>,
}
