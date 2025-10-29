// Custom database queries
// Add complex queries here that don't fit in route handlers

use crate::error::Result;
use sqlx::SqlitePool;

#[allow(dead_code)]
pub async fn health_check_db(pool: &SqlitePool) -> Result<bool> {
    sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(true)
}
