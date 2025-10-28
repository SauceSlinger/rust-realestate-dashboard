use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use crate::models::{Property, CreateProperty, UpdateProperty};
use crate::db;

pub async fn list_properties(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Property>>, StatusCode> {
    db::get_properties(&pool)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_property(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Property>, StatusCode> {
    db::get_property(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn create_property(
    State(pool): State<SqlitePool>,
    Json(property): Json<CreateProperty>,
) -> Result<Json<i64>, StatusCode> {
    db::create_property(&pool, property)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_property(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(property): Json<UpdateProperty>,
) -> Result<StatusCode, StatusCode> {
    let updated = db::update_property(&pool, id, property)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if updated {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_property(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let deleted = db::delete_property(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
