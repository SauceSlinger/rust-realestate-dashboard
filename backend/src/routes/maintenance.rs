use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use crate::error::{AppError, Result};
use crate::models::{MaintenanceRecord, CreateMaintenance, UpdateMaintenance};

pub async fn list_maintenance(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<MaintenanceRecord>>> {
    let records = sqlx::query_as::<_, MaintenanceRecord>(
        "SELECT * FROM maintenance_records ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(records))
}

pub async fn create_maintenance(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateMaintenance>,
) -> Result<(StatusCode, Json<MaintenanceRecord>)> {
    let result = sqlx::query(
        r#"
        INSERT INTO maintenance_records (
            property_id, title, description, priority, status,
            cost, scheduled_date, contractor, notes
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(payload.property_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.priority)
    .bind(&payload.status)
    .bind(payload.cost)
    .bind(payload.scheduled_date)
    .bind(&payload.contractor)
    .bind(&payload.notes)
    .execute(&pool)
    .await?;

    let record = sqlx::query_as::<_, MaintenanceRecord>(
        "SELECT * FROM maintenance_records WHERE id = ?"
    )
    .bind(result.last_insert_rowid())
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(record)))
}

pub async fn update_maintenance(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateMaintenance>,
) -> Result<Json<MaintenanceRecord>> {
    sqlx::query("SELECT id FROM maintenance_records WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Maintenance record with id {} not found", id)))?;

    let mut query = String::from("UPDATE maintenance_records SET ");
    let mut updates = Vec::new();
    
    if payload.title.is_some() { updates.push("title = ?"); }
    if payload.description.is_some() { updates.push("description = ?"); }
    if payload.priority.is_some() { updates.push("priority = ?"); }
    if payload.status.is_some() { updates.push("status = ?"); }
    if payload.cost.is_some() { updates.push("cost = ?"); }
    if payload.scheduled_date.is_some() { updates.push("scheduled_date = ?"); }
    if payload.completed_date.is_some() { updates.push("completed_date = ?"); }
    if payload.contractor.is_some() { updates.push("contractor = ?"); }
    if payload.notes.is_some() { updates.push("notes = ?"); }
    
    updates.push("updated_at = CURRENT_TIMESTAMP");
    
    query.push_str(&updates.join(", "));
    query.push_str(" WHERE id = ?");

    let mut q = sqlx::query(&query);
    
    if let Some(v) = &payload.title { q = q.bind(v); }
    if let Some(v) = &payload.description { q = q.bind(v); }
    if let Some(v) = &payload.priority { q = q.bind(v); }
    if let Some(v) = &payload.status { q = q.bind(v); }
    if let Some(v) = payload.cost { q = q.bind(v); }
    if let Some(v) = payload.scheduled_date { q = q.bind(v); }
    if let Some(v) = payload.completed_date { q = q.bind(v); }
    if let Some(v) = &payload.contractor { q = q.bind(v); }
    if let Some(v) = &payload.notes { q = q.bind(v); }
    
    q = q.bind(id);
    q.execute(&pool).await?;

    let record = sqlx::query_as::<_, MaintenanceRecord>(
        "SELECT * FROM maintenance_records WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(record))
}
