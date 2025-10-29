use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CalendarEvent {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub event_type: String, // maintenance, rent_due, inspection, lease_renewal
    pub property_id: Option<i64>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub reminder_minutes: Option<i32>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEvent {
    pub title: String,
    pub description: Option<String>,
    pub event_type: String,
    pub property_id: Option<i64>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub reminder_minutes: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEvent {
    pub title: Option<String>,
    pub description: Option<String>,
    pub event_type: Option<String>,
    pub property_id: Option<i64>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub reminder_minutes: Option<i32>,
    pub completed: Option<bool>,
}
