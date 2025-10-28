use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use serde::Serialize;
use crate::models::CreateMarketData;
use crate::db;

#[derive(Serialize)]
pub struct ScraperResponse {
    message: String,
    count: usize,
}

pub async fn scrape_market_data(
    State(pool): State<SqlitePool>,
) -> Result<Json<ScraperResponse>, StatusCode> {
    // Simulated market data scraping
    // In a real application, this would scrape from actual real estate websites
    let mock_data = vec![
        CreateMarketData {
            location: "San Francisco, CA".to_string(),
            median_price: 1250000.0,
            average_price_per_sqft: 950.0,
            inventory_count: 523,
            days_on_market: 28,
            data_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            source: "Mock Data".to_string(),
        },
        CreateMarketData {
            location: "Austin, TX".to_string(),
            median_price: 550000.0,
            average_price_per_sqft: 325.0,
            inventory_count: 892,
            days_on_market: 35,
            data_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            source: "Mock Data".to_string(),
        },
        CreateMarketData {
            location: "Seattle, WA".to_string(),
            median_price: 825000.0,
            average_price_per_sqft: 575.0,
            inventory_count: 654,
            days_on_market: 24,
            data_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            source: "Mock Data".to_string(),
        },
    ];

    let count = mock_data.len();
    
    for data in mock_data {
        db::create_market_data(&pool, data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(Json(ScraperResponse {
        message: "Market data scraped successfully".to_string(),
        count,
    }))
}
