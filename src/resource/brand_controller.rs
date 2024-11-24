use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::brand_models::{CreateBrand, UpdateBrand};
use crate::service::brand_service;

const MAX_LIMIT: u32 = 100;

pub fn router(pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Router {
    Router::new()
        .route("/v1/brand", get(get_all))
        // Single operations
        .route("/v1/brand", post(self::create_one))
        .route("/v1/brand/:id", get(self::get_one))
        .route("/v1/brand/:id", patch(self::update_one))
        .route("/v1/brand/:id", delete(self::delete_one))
        // Bulk operations
        .route("/v1/brand/bulk", post(self::create_many))
        .route("/v1/brand/bulk", delete(self::delete_many))
        .with_state(pool)
}

#[derive(Deserialize)]
pub struct GetParams {
    offset: Option<u32>,
    limit: Option<u32>,
    sort_by: Option<String>,
    sort_order: Option<String>,
    filter_by: Option<String>,
    filter_term: Option<String>,
}

pub async fn get_all(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Query(params): Query<GetParams>,
) -> Response {
    match brand_service::get_brands(
        pool,
        params.offset.unwrap_or(0),
        params.limit.unwrap_or(10).min(MAX_LIMIT),
        params.sort_by.unwrap_or_else(|| String::from("name")),
        params.sort_order.unwrap_or_else(|| String::from("asc")),
        params.filter_by.unwrap_or_else(|| String::from("")),
        params.filter_term.unwrap_or_else(|| String::from("")),
    ) {
        Ok(brands) => (StatusCode::OK, Json(brands)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn get_one(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Path(brand_id): Path<Uuid>,
) -> Response {
    match brand_service::get_brand(pool, brand_id) {
        Ok(brand) => (StatusCode::OK, Json(brand)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn create_one(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Json(payload): Json<CreateBrand>,
) -> Response {
    match brand_service::create_brand(pool, payload) {
        Ok(brand) => (StatusCode::OK, Json(brand)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn create_many(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Json(payload): Json<Vec<CreateBrand>>,
) -> Response {
    match brand_service::create_brands(pool, payload) {
        Ok(brand) => (StatusCode::OK, Json(brand)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn update_one(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Path(brand_id): Path<Uuid>,
    Json(payload): Json<UpdateBrand>,
) -> Response {
    match brand_service::update_brand(pool, brand_id, payload) {
        Ok(brand) => (StatusCode::OK, Json(brand)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn delete_one(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Path(brand_id): Path<Uuid>,
) -> Response {
    match brand_service::delete_brand(pool, brand_id) {
        Ok(count) => get_status_code_for_count(count).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn delete_many(
    State(pool): State<Arc<Pool<ConnectionManager<PgConnection>>>>,
    Json(brands_ids): Json<Vec<Uuid>>,
) -> Response {
    match brand_service::delete_brands(pool, brands_ids) {
        Ok(count) => get_status_code_for_count(count).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

fn get_status_code_for_count(count: usize) -> StatusCode {
    if count > 0 {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
