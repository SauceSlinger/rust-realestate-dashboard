// Custom database queries
// Add complex queries here that don't fit in route handlers

use sqlx::SqlitePool;
use crate::error::Result;

pub async fn health_check_db(pool: &SqlitePool) -> Result<bool> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await?;
    Ok(true)
}
