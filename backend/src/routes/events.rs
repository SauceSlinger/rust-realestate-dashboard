use crate::error::{AppError, Result};
use crate::models::{CalendarEvent, CreateEvent, UpdateEvent};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

pub async fn list_events(State(pool): State<SqlitePool>) -> Result<Json<Vec<CalendarEvent>>> {
    let events =
        sqlx::query_as::<_, CalendarEvent>("SELECT * FROM calendar_events ORDER BY start_time ASC")
            .fetch_all(&pool)
            .await?;

    Ok(Json(events))
}

pub async fn get_event(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<CalendarEvent>> {
    let event = sqlx::query_as::<_, CalendarEvent>("SELECT * FROM calendar_events WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Event with id {} not found", id)))?;

    Ok(Json(event))
}

pub async fn create_event(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateEvent>,
) -> Result<(StatusCode, Json<CalendarEvent>)> {
    let result = sqlx::query(
        r#"
        INSERT INTO calendar_events (
            title, description, event_type, property_id,
            start_time, end_time, reminder_minutes
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.event_type)
    .bind(payload.property_id)
    .bind(payload.start_time)
    .bind(payload.end_time)
    .bind(payload.reminder_minutes)
    .execute(&pool)
    .await?;

    let event = sqlx::query_as::<_, CalendarEvent>("SELECT * FROM calendar_events WHERE id = ?")
        .bind(result.last_insert_rowid())
        .fetch_one(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(event)))
}

pub async fn update_event(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateEvent>,
) -> Result<Json<CalendarEvent>> {
    sqlx::query("SELECT id FROM calendar_events WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Event with id {} not found", id)))?;

    let mut query = String::from("UPDATE calendar_events SET ");
    let mut updates = Vec::new();

    if payload.title.is_some() {
        updates.push("title = ?");
    }
    if payload.description.is_some() {
        updates.push("description = ?");
    }
    if payload.event_type.is_some() {
        updates.push("event_type = ?");
    }
    if payload.property_id.is_some() {
        updates.push("property_id = ?");
    }
    if payload.start_time.is_some() {
        updates.push("start_time = ?");
    }
    if payload.end_time.is_some() {
        updates.push("end_time = ?");
    }
    if payload.reminder_minutes.is_some() {
        updates.push("reminder_minutes = ?");
    }
    if payload.completed.is_some() {
        updates.push("completed = ?");
    }

    updates.push("updated_at = CURRENT_TIMESTAMP");

    query.push_str(&updates.join(", "));
    query.push_str(" WHERE id = ?");

    let mut q = sqlx::query(&query);

    if let Some(v) = &payload.title {
        q = q.bind(v);
    }
    if let Some(v) = &payload.description {
        q = q.bind(v);
    }
    if let Some(v) = &payload.event_type {
        q = q.bind(v);
    }
    if let Some(v) = payload.property_id {
        q = q.bind(v);
    }
    if let Some(v) = payload.start_time {
        q = q.bind(v);
    }
    if let Some(v) = payload.end_time {
        q = q.bind(v);
    }
    if let Some(v) = payload.reminder_minutes {
        q = q.bind(v);
    }
    if let Some(v) = payload.completed {
        q = q.bind(v);
    }

    q = q.bind(id);
    q.execute(&pool).await?;

    let event = sqlx::query_as::<_, CalendarEvent>("SELECT * FROM calendar_events WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(event))
}

pub async fn delete_event(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    let result = sqlx::query("DELETE FROM calendar_events WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Event with id {} not found",
            id
        )));
    }

    Ok(StatusCode::NO_CONTENT)
}
