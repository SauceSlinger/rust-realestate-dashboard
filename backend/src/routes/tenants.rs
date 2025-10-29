use crate::error::{AppError, Result};
use crate::models::{CreateTenant, Tenant, UpdateTenant};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

pub async fn list_tenants(State(pool): State<SqlitePool>) -> Result<Json<Vec<Tenant>>> {
    let tenants = sqlx::query_as::<_, Tenant>("SELECT * FROM tenants ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;

    Ok(Json(tenants))
}

pub async fn get_tenant(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Tenant>> {
    let tenant = sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Tenant with id {} not found", id)))?;

    Ok(Json(tenant))
}

pub async fn create_tenant(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTenant>,
) -> Result<(StatusCode, Json<Tenant>)> {
    let result = sqlx::query(
        r#"
        INSERT INTO tenants (
            property_id, first_name, last_name, email, phone,
            lease_start, lease_end, monthly_rent, deposit_amount,
            status, notes
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(payload.property_id)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.email)
    .bind(&payload.phone)
    .bind(payload.lease_start)
    .bind(payload.lease_end)
    .bind(payload.monthly_rent)
    .bind(payload.deposit_amount)
    .bind(&payload.status)
    .bind(&payload.notes)
    .execute(&pool)
    .await?;

    let tenant = sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE id = ?")
        .bind(result.last_insert_rowid())
        .fetch_one(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(tenant)))
}

pub async fn update_tenant(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTenant>,
) -> Result<Json<Tenant>> {
    sqlx::query("SELECT id FROM tenants WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Tenant with id {} not found", id)))?;

    let mut query = String::from("UPDATE tenants SET ");
    let mut updates = Vec::new();

    if payload.property_id.is_some() {
        updates.push("property_id = ?");
    }
    if payload.first_name.is_some() {
        updates.push("first_name = ?");
    }
    if payload.last_name.is_some() {
        updates.push("last_name = ?");
    }
    if payload.email.is_some() {
        updates.push("email = ?");
    }
    if payload.phone.is_some() {
        updates.push("phone = ?");
    }
    if payload.lease_start.is_some() {
        updates.push("lease_start = ?");
    }
    if payload.lease_end.is_some() {
        updates.push("lease_end = ?");
    }
    if payload.monthly_rent.is_some() {
        updates.push("monthly_rent = ?");
    }
    if payload.deposit_amount.is_some() {
        updates.push("deposit_amount = ?");
    }
    if payload.status.is_some() {
        updates.push("status = ?");
    }
    if payload.notes.is_some() {
        updates.push("notes = ?");
    }

    updates.push("updated_at = CURRENT_TIMESTAMP");

    query.push_str(&updates.join(", "));
    query.push_str(" WHERE id = ?");

    let mut q = sqlx::query(&query);

    if let Some(v) = payload.property_id {
        q = q.bind(v);
    }
    if let Some(v) = &payload.first_name {
        q = q.bind(v);
    }
    if let Some(v) = &payload.last_name {
        q = q.bind(v);
    }
    if let Some(v) = &payload.email {
        q = q.bind(v);
    }
    if let Some(v) = &payload.phone {
        q = q.bind(v);
    }
    if let Some(v) = payload.lease_start {
        q = q.bind(v);
    }
    if let Some(v) = payload.lease_end {
        q = q.bind(v);
    }
    if let Some(v) = payload.monthly_rent {
        q = q.bind(v);
    }
    if let Some(v) = payload.deposit_amount {
        q = q.bind(v);
    }
    if let Some(v) = &payload.status {
        q = q.bind(v);
    }
    if let Some(v) = &payload.notes {
        q = q.bind(v);
    }

    q = q.bind(id);
    q.execute(&pool).await?;

    let tenant = sqlx::query_as::<_, Tenant>("SELECT * FROM tenants WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(tenant))
}

pub async fn delete_tenant(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    let result = sqlx::query("DELETE FROM tenants WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Tenant with id {} not found",
            id
        )));
    }

    Ok(StatusCode::NO_CONTENT)
}
