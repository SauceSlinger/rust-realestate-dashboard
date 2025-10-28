use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use crate::models::{Reminder, CreateReminder, UpdateReminder};
use crate::db;

pub async fn list_reminders(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Reminder>>, StatusCode> {
    db::get_reminders(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_reminder(
    State(pool): State<SqlitePool>,
    Json(reminder): Json<CreateReminder>,
) -> Result<Json<i64>, StatusCode> {
    db::create_reminder(&pool, reminder)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_reminder(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(reminder): Json<UpdateReminder>,
) -> Result<StatusCode, StatusCode> {
    let updated = db::update_reminder(&pool, id, reminder)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if updated {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_reminder(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let deleted = db::delete_reminder(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
