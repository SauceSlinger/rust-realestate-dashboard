use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tenant {
    pub id: Option<i64>,
    pub property_id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub lease_start: String,
    pub lease_end: String,
    pub monthly_rent: f64,
    pub deposit: f64,
    pub status: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTenant {
    pub property_id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub lease_start: String,
    pub lease_end: String,
    pub monthly_rent: f64,
    pub deposit: f64,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTenant {
    pub property_id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub lease_start: Option<String>,
    pub lease_end: Option<String>,
    pub monthly_rent: Option<f64>,
    pub deposit: Option<f64>,
    pub status: Option<String>,
}
