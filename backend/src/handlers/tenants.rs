use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use serde::Deserialize;
use crate::models::{Tenant, CreateTenant, UpdateTenant};
use crate::db;

#[derive(Deserialize)]
pub struct TenantQuery {
    property_id: Option<i64>,
}

pub async fn list_tenants(
    State(pool): State<SqlitePool>,
    Query(query): Query<TenantQuery>,
) -> Result<Json<Vec<Tenant>>, StatusCode> {
    db::get_tenants(&pool, query.property_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn create_tenant(
    State(pool): State<SqlitePool>,
    Json(tenant): Json<CreateTenant>,
) -> Result<Json<i64>, StatusCode> {
    db::create_tenant(&pool, tenant)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_tenant(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Json(tenant): Json<UpdateTenant>,
) -> Result<StatusCode, StatusCode> {
    let updated = db::update_tenant(&pool, id, tenant)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if updated {
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_tenant(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    let deleted = db::delete_tenant(&pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
