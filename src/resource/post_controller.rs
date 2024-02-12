use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, patch};
use axum::routing::delete;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use showroom_backend_api::models::models::Post;
use showroom_backend_api::models::models::CreatePost;

use crate::service::post_service;

pub fn router(pool: Pool<ConnectionManager<PgConnection>>) -> Router {
    Router::new()
        .route("/v1/post", get(get_all))
        .route("/v1/post", post(create))
        .route("/v1/post", patch(update))
        .route("/v1/post/:id", get(get_one))
        .route("/v1/post/:id", delete(delete_one))
        .with_state(pool)
}

pub async fn get_all(State(pool): State<Pool<ConnectionManager<PgConnection>>>) -> Response {
    match post_service::get_all_posts(pool, 100) {
        Some(posts) => (StatusCode::OK, Json(posts)).into_response(),
        None => (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}

pub async fn get_one(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Path(post_id): Path<Uuid>) -> Response {
    match post_service::get_post(pool, post_id) {
        Some(result) => (StatusCode::OK, Json(result)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn create(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Json(payload): Json<CreatePost>) -> Response {
    match post_service::create_post(pool, payload) {
        Some(post) => (StatusCode::CREATED, Json(post)).into_response(),
        None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn update(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Json(payload): Json<Post>) -> Response {
    let update_count = post_service::update_post(pool, payload);

    if update_count > 0 {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn delete_one(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Path(post_id): Path<Uuid>) -> Response {
    let delete_count = post_service::delete_post(pool, post_id);

    if delete_count > 0 {
        StatusCode::NO_CONTENT.into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}