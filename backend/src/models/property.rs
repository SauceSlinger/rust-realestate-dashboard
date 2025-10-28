use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Property {
    pub id: Option<i64>,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub property_type: String,
    pub bedrooms: i32,
    pub bathrooms: f32,
    pub square_feet: i32,
    pub purchase_price: f64,
    pub current_value: f64,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProperty {
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub property_type: String,
    pub bedrooms: i32,
    pub bathrooms: f32,
    pub square_feet: i32,
    pub purchase_price: f64,
    pub current_value: f64,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProperty {
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
    pub status: Option<String>,
    pub notes: Option<String>,
}
