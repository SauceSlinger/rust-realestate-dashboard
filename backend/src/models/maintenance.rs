use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MaintenanceRecord {
    pub id: i64,
    pub property_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub priority: String, // low, medium, high, urgent
    pub status: String,   // pending, in_progress, completed, cancelled
    pub cost: Option<f64>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub completed_date: Option<DateTime<Utc>>,
    pub contractor: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMaintenance {
    pub property_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub priority: String,
    pub status: String,
    pub cost: Option<f64>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub contractor: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMaintenance {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub cost: Option<f64>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub completed_date: Option<DateTime<Utc>>,
    pub contractor: Option<String>,
    pub notes: Option<String>,
}
