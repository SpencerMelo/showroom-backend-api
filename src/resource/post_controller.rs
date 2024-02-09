use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post, patch};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::prelude::*;
use uuid::Uuid;

use showroom_backend_api::models::models::Post;
use showroom_backend_api::models::models::CreatePost;
use showroom_backend_api::schema::posts::dsl::*;

pub fn router(pool: Pool<ConnectionManager<PgConnection>>) -> Router {
    Router::new()
        .route("/v1/post", get(get_all))
        .route("/v1/post", post(create))
        .route("/v1/post", patch(update))
        .route("/v1/post/:id", get(get_one))
        .with_state(pool)
}

pub async fn get_all(State(pool): State<Pool<ConnectionManager<PgConnection>>>) -> Response {
    let connection: &mut PgConnection = &mut pool.get().unwrap();

    let results: Vec<Post> = posts
        .filter(published.eq(true))
        .limit(20)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    (StatusCode::OK, Json(results)).into_response()
}

pub async fn get_one(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Path(post_id): Path<Uuid>) -> Response {
    let connection: &mut PgConnection = &mut pool.get().unwrap();

    let result = posts
        .filter(id.eq(post_id))
        .select(Post::as_select())
        .first(connection)
        .optional();

    match result {
        Ok(Some(result)) => (StatusCode::OK, Json(result)).into_response(),
        Ok(None) | Err(_) => (StatusCode::NOT_FOUND, String::from("")).into_response(),
    }
}

pub async fn create(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Json(payload): Json<CreatePost>) -> Response {
    let connection: &mut PgConnection = &mut pool.get().unwrap();

    let new_post: Post = Post {
        id: Uuid::new_v4(),
        brand: payload.brand,
        model: payload.model,
        version: payload.version,
        engine: payload.engine,
        transmission: payload.transmission,
        year: payload.year,
        mileage: payload.mileage,
        color: payload.color,
        body: payload.body,
        armored: payload.armored,
        exchange: payload.exchange,
        price: payload.price,
        thumbnail_url: payload.thumbnail_url,
        author: payload.author,
        published: true,
    };

    let result: Post = diesel::insert_into(posts)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(connection).expect("Error saving new post");

    (StatusCode::CREATED, Json(result)).into_response()
}

pub async fn update(State(pool): State<Pool<ConnectionManager<PgConnection>>>, Json(payload): Json<Post>) -> Response {
    let connection: &mut PgConnection = &mut pool.get().unwrap();

    diesel::update(posts)
        .filter(id.eq(payload.id))
        .set(payload)
        .execute(connection)
        .expect("Error patching post");

    StatusCode::NO_CONTENT.into_response()
}