use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use serde::Deserialize;
use crate::models::{MarketData, CreateMarketData, MarketTrend, TrendPoint};
use crate::db;

#[derive(Deserialize)]
pub struct MarketDataQuery {
    location: Option<String>,
}

pub async fn list_market_data(
    State(pool): State<SqlitePool>,
    Query(query): Query<MarketDataQuery>,
) -> Result<Json<Vec<MarketData>>, StatusCode> {
    db::get_market_data(&pool, query.location)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_market_data(
    State(pool): State<SqlitePool>,
    Json(data): Json<CreateMarketData>,
) -> Result<Json<i64>, StatusCode> {
    db::create_market_data(&pool, data)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_trends(
    State(pool): State<SqlitePool>,
    Query(query): Query<MarketDataQuery>,
) -> Result<Json<Vec<MarketTrend>>, StatusCode> {
    let location = query.location.unwrap_or_else(|| "All".to_string());
    
    let data = db::get_market_data(&pool, Some(location.clone()))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let trend_data: Vec<TrendPoint> = data.iter().map(|d| TrendPoint {
        date: d.data_date.clone(),
        median_price: d.median_price,
        average_price_per_sqft: d.average_price_per_sqft,
    }).collect();
    
    let trends = vec![MarketTrend {
        location,
        trend_data,
    }];
    
    Ok(Json(trends))
}
