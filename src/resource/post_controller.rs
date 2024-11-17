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

use showroom_api::models::models::CreatePost;
use showroom_api::models::models::Post;

use crate::service::post_service;

const MAX_LIMIT: u32 = 100;

pub fn router(pool: Pool<ConnectionManager<PgConnection>>) -> Router {
    Router::new()
        .route("/v1/post", get(get_all))
        .route("/v1/post", post(create))
        .route("/v1/post", patch(update))
        .route("/v1/post/:id", get(get_one))
        .route("/v1/post/:id", delete(delete_one))
        .with_state(pool)
}

#[derive(Deserialize)]
pub struct Pagination {
    offset: Option<u32>,
    limit: Option<u32>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

pub async fn get_all(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Query(pagination): Query<Pagination>,
) -> Response {
    match post_service::get_posts(
        pool,
        pagination.offset.unwrap_or(0),
        pagination.limit.unwrap_or(10).min(MAX_LIMIT),
        pagination.sort_by.unwrap_or_else(|| String::from("model")),
        pagination.sort_order.unwrap_or_else(|| String::from("asc")),
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

pub async fn create(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(payload): Json<CreatePost>,
) -> Response {
    match post_service::create_post(pool, payload) {
        Ok(post) => (StatusCode::CREATED, Json(post)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}

pub async fn update(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Json(payload): Json<Post>,
) -> Response {
    match post_service::update_post(pool, payload) {
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

fn get_status_code_for_count(count: usize) -> StatusCode {
    if count > 0 {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
