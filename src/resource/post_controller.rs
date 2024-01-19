use axum::{Json, Router};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::prelude::*;

use showroom_backend_api::models::models::Post;
use showroom_backend_api::schema::post::dsl::*;

pub fn router(pool: Pool<ConnectionManager<PgConnection>>) -> Router {
     Router::new()
        .route("/v1/post", get(all))
        .with_state(pool)
}

pub async fn all(State(pool): State<Pool<ConnectionManager<PgConnection>>>) -> Result<Json<Vec<Post>>, (StatusCode, String)> {
    let connection: &mut PgConnection = &mut pool.get().unwrap();

    let results: Vec<Post> = post
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    Ok(Json(results))
}