use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Property {
    pub id: i64,
    pub title: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub property_type: String, // residential, commercial, etc.
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<f32>,
    pub square_feet: Option<i32>,
    pub purchase_price: Option<f64>,
    pub current_value: Option<f64>,
    pub monthly_rent: Option<f64>,
    pub status: String, // occupied, vacant, maintenance
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProperty {
    pub title: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub property_type: String,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<f32>,
    pub square_feet: Option<i32>,
    pub purchase_price: Option<f64>,
    pub current_value: Option<f64>,
    pub monthly_rent: Option<f64>,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProperty {
    pub title: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip_code: Option<String>,
    pub property_type: Option<String>,
    pub bedrooms: Option<i32>,
    pub bathrooms: Option<f32>,
    pub square_feet: Option<i32>,
    pub purchase_price: Option<f64>,
    pub current_value: Option<f64>,
    pub monthly_rent: Option<f64>,
    pub status: Option<String>,
    pub notes: Option<String>,
}
