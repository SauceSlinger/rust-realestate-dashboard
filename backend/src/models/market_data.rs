use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketData {
    pub id: Option<i64>,
    pub location: String,
    pub median_price: f64,
    pub average_price_per_sqft: f64,
    pub inventory_count: i32,
    pub days_on_market: i32,
    pub data_date: String,
    pub source: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMarketData {
    pub location: String,
    pub median_price: f64,
    pub average_price_per_sqft: f64,
    pub inventory_count: i32,
    pub days_on_market: i32,
    pub data_date: String,
    pub source: String,
}

#[derive(Debug, Serialize)]
pub struct MarketTrend {
    pub location: String,
    pub trend_data: Vec<TrendPoint>,
}

#[derive(Debug, Serialize)]
pub struct TrendPoint {
    pub date: String,
    pub median_price: f64,
    pub average_price_per_sqft: f64,
}
