use crate::error::{AppError, Result};
use crate::models::{CreateProperty, Property, UpdateProperty};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

pub async fn list_properties(State(pool): State<SqlitePool>) -> Result<Json<Vec<Property>>> {
    let properties =
        sqlx::query_as::<_, Property>("SELECT * FROM properties ORDER BY created_at DESC")
            .fetch_all(&pool)
            .await?;

    Ok(Json(properties))
}

pub async fn get_property(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Property>> {
    let property = sqlx::query_as::<_, Property>("SELECT * FROM properties WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Property with id {} not found", id)))?;

    Ok(Json(property))
}

pub async fn create_property(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateProperty>,
) -> Result<(StatusCode, Json<Property>)> {
    let result = sqlx::query(
        r#"
        INSERT INTO properties (
            title, address, city, state, zip_code, property_type,
            bedrooms, bathrooms, square_feet, purchase_price,
            current_value, monthly_rent, status, notes
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.address)
    .bind(&payload.city)
    .bind(&payload.state)
    .bind(&payload.zip_code)
    .bind(&payload.property_type)
    .bind(payload.bedrooms)
    .bind(payload.bathrooms)
    .bind(payload.square_feet)
    .bind(payload.purchase_price)
    .bind(payload.current_value)
    .bind(payload.monthly_rent)
    .bind(&payload.status)
    .bind(&payload.notes)
    .execute(&pool)
    .await?;

    let property = sqlx::query_as::<_, Property>("SELECT * FROM properties WHERE id = ?")
        .bind(result.last_insert_rowid())
        .fetch_one(&pool)
        .await?;

    Ok((StatusCode::CREATED, Json(property)))
}

pub async fn update_property(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateProperty>,
) -> Result<Json<Property>> {
    // First verify the property exists
    sqlx::query("SELECT id FROM properties WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Property with id {} not found", id)))?;

    // Build dynamic update query
    let mut query = String::from("UPDATE properties SET ");
    let mut updates = Vec::new();

    if payload.title.is_some() {
        updates.push("title = ?");
    }
    if payload.address.is_some() {
        updates.push("address = ?");
    }
    if payload.city.is_some() {
        updates.push("city = ?");
    }
    if payload.state.is_some() {
        updates.push("state = ?");
    }
    if payload.zip_code.is_some() {
        updates.push("zip_code = ?");
    }
    if payload.property_type.is_some() {
        updates.push("property_type = ?");
    }
    if payload.bedrooms.is_some() {
        updates.push("bedrooms = ?");
    }
    if payload.bathrooms.is_some() {
        updates.push("bathrooms = ?");
    }
    if payload.square_feet.is_some() {
        updates.push("square_feet = ?");
    }
    if payload.purchase_price.is_some() {
        updates.push("purchase_price = ?");
    }
    if payload.current_value.is_some() {
        updates.push("current_value = ?");
    }
    if payload.monthly_rent.is_some() {
        updates.push("monthly_rent = ?");
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

    if let Some(v) = &payload.title {
        q = q.bind(v);
    }
    if let Some(v) = &payload.address {
        q = q.bind(v);
    }
    if let Some(v) = &payload.city {
        q = q.bind(v);
    }
    if let Some(v) = &payload.state {
        q = q.bind(v);
    }
    if let Some(v) = &payload.zip_code {
        q = q.bind(v);
    }
    if let Some(v) = &payload.property_type {
        q = q.bind(v);
    }
    if let Some(v) = payload.bedrooms {
        q = q.bind(v);
    }
    if let Some(v) = payload.bathrooms {
        q = q.bind(v);
    }
    if let Some(v) = payload.square_feet {
        q = q.bind(v);
    }
    if let Some(v) = payload.purchase_price {
        q = q.bind(v);
    }
    if let Some(v) = payload.current_value {
        q = q.bind(v);
    }
    if let Some(v) = payload.monthly_rent {
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

    let property = sqlx::query_as::<_, Property>("SELECT * FROM properties WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Json(property))
}

pub async fn delete_property(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode> {
    let result = sqlx::query("DELETE FROM properties WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Property with id {} not found",
            id
        )));
    }

    Ok(StatusCode::NO_CONTENT)
}
