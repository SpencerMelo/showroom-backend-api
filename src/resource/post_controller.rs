use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::delete;
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::post_models::{CreatePost, UpdatePost};
use crate::service::post_service;

const MAX_LIMIT: u32 = 100;

pub fn router(pool: Pool<ConnectionManager<PgConnection>>) -> Router {
    Router::new()
        .route("/v1/post", get(get_all))
        // Single operations
        .route("/v1/post", post(self::create_one))
        .route("/v1/post/:id", get(self::get_one))
        .route("/v1/post/:id", delete(self::delete_one))
        .route("/v1/post/:id", patch(self::update_one))
        // Bulk operations
        .route("/v1/post/bulk", post(self::create_many))
        .route("/v1/post/bulk", delete(self::delete_many))
        // Route state
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
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Query(params): Query<GetParams>,
) -> Response {
    match post_service::get_posts(
        pool,
        params.offset.unwrap_or(0),
        params.limit.unwrap_or(10).min(MAX_LIMIT),
        params.sort_by.unwrap_or_else(|| String::from("model")),
        params.sort_order.unwrap_or_else(|| String::from("asc")),
        params.filter_by.unwrap_or_else(|| String::from("")),
        params.filter_term.unwrap_or_else(|| String::from("")),
    ) {
        Ok(posts) => (StatusCode::OK, Json(posts)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn get_one(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(post_id): Path<Uuid>,
) -> Response {
    match post_service::get_post(pool, post_id) {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn create_one(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(payload): Json<CreatePost>,
) -> Response {
    match post_service::create_post(pool, payload) {
        Ok(post) => (StatusCode::CREATED, Json(post)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn create_many(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(payload): Json<Vec<CreatePost>>,
) -> Response {
    match post_service::create_posts(pool, payload) {
        Ok(posts) => (StatusCode::CREATED, Json(posts)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn update_one(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(post_id): Path<Uuid>,
    Json(payload): Json<UpdatePost>,
) -> Response {
    match post_service::update_post(pool, post_id, payload) {
        Ok(count) => get_status_code_for_count(count).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn delete_one(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(post_id): Path<Uuid>,
) -> Response {
    match post_service::delete_post(pool, post_id) {
        Ok(count) => get_status_code_for_count(count).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn delete_many(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(posts_ids): Json<Vec<Uuid>>,
) -> Response {
    match post_service::delete_posts(pool, posts_ids) {
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
