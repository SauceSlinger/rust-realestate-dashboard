use crate::error::Result;
use crate::models::{MarketAnalytics, TrendData, TrendPoint};
use crate::scraper;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

pub async fn get_trends(State(pool): State<SqlitePool>) -> Result<Json<Vec<TrendData>>> {
    let data = sqlx::query_as::<_, (String, String, Option<f64>, Option<i32>)>(
        r#"
        SELECT location, recorded_date, median_price, inventory_count
        FROM market_data
        ORDER BY location, recorded_date DESC
        "#,
    )
    .fetch_all(&pool)
    .await?;

    // Group by location
    let mut trends_map: std::collections::HashMap<String, Vec<TrendPoint>> =
        std::collections::HashMap::new();

    for (location, date_str, median_price, inventory_count) in data {
        let date = chrono::DateTime::parse_from_rfc3339(&date_str)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());

        trends_map
            .entry(location.clone())
            .or_default()
            .push(TrendPoint {
                date,
                median_price,
                inventory_count,
            });
    }

    let trends: Vec<TrendData> = trends_map
        .into_iter()
        .map(|(location, time_series)| TrendData {
            location,
            time_series,
        })
        .collect();

    Ok(Json(trends))
}

pub async fn get_analytics(State(pool): State<SqlitePool>) -> Result<Json<MarketAnalytics>> {
    // Get total properties
    let total_properties: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM properties")
        .fetch_one(&pool)
        .await?;

    // Get total value
    let total_value: Option<f64> = sqlx::query_scalar(
        "SELECT SUM(current_value) FROM properties WHERE current_value IS NOT NULL",
    )
    .fetch_one(&pool)
    .await?;

    // Get average rent
    let average_rent: Option<f64> = sqlx::query_scalar(
        "SELECT AVG(monthly_rent) FROM properties WHERE monthly_rent IS NOT NULL",
    )
    .fetch_one(&pool)
    .await?;

    // Get occupancy rate
    let occupied: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM properties WHERE status = 'occupied'")
            .fetch_one(&pool)
            .await?;

    let occupancy_rate = if total_properties > 0 {
        (occupied as f32 / total_properties as f32) * 100.0
    } else {
        0.0
    };

    // Get market trends
    let data = sqlx::query_as::<_, (String, String, Option<f64>, Option<i32>)>(
        r#"
        SELECT location, recorded_date, median_price, inventory_count
        FROM market_data
        ORDER BY location, recorded_date DESC
        LIMIT 100
        "#,
    )
    .fetch_all(&pool)
    .await?;

    let mut trends_map: std::collections::HashMap<String, Vec<TrendPoint>> =
        std::collections::HashMap::new();

    for (location, date_str, median_price, inventory_count) in data {
        let date = chrono::DateTime::parse_from_rfc3339(&date_str)
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now());

        trends_map
            .entry(location.clone())
            .or_default()
            .push(TrendPoint {
                date,
                median_price,
                inventory_count,
            });
    }

    let market_trends: Vec<TrendData> = trends_map
        .into_iter()
        .map(|(location, time_series)| TrendData {
            location,
            time_series,
        })
        .collect();

    Ok(Json(MarketAnalytics {
        total_properties,
        total_value: total_value.unwrap_or(0.0),
        average_rent: average_rent.unwrap_or(0.0),
        occupancy_rate,
        market_trends,
    }))
}

pub async fn trigger_scrape(State(pool): State<SqlitePool>) -> Result<StatusCode> {
    // Trigger the scraper in a background task
    tokio::spawn(async move {
        if let Err(e) = scraper::scrape_market_data(&pool).await {
            tracing::error!("Scraping failed: {:?}", e);
        }
    });

    Ok(StatusCode::ACCEPTED)
}
