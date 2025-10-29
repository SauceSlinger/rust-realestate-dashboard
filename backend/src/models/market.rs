use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct MarketData {
    pub id: i64,
    pub location: String, // city or zip code
    pub median_price: Option<f64>,
    pub average_price: Option<f64>,
    pub inventory_count: Option<i32>,
    pub days_on_market: Option<f32>,
    pub price_change_percent: Option<f32>,
    pub data_source: String,
    pub recorded_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendData {
    pub location: String,
    pub time_series: Vec<TrendPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrendPoint {
    pub date: DateTime<Utc>,
    pub median_price: Option<f64>,
    pub inventory_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketAnalytics {
    pub total_properties: i64,
    pub total_value: f64,
    pub average_rent: f64,
    pub occupancy_rate: f32,
    pub market_trends: Vec<TrendData>,
}
