use showroom_backend_api::models::models::{CreatePost, Post};
use showroom_backend_api::schema::posts::dsl::*;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use log::{error, info};

use uuid::Uuid;

pub fn get_all_posts(pool: Pool<ConnectionManager<PgConnection>>, limit: u8) -> Option<Vec<Post>> {
    info!("Get all posts, limited to {}", limit);
    let connection: &mut PgConnection = &mut pool.get().unwrap();

    let post_list = posts
        .filter(published.eq(true))
        .limit(limit.into())
        .select(Post::as_select())
        .load(connection);

    match post_list {
        Ok(post_list) => Some(post_list),
        Err(err) => {
            error!("Unable to retrieve posts, error: {}", err.to_string());
            None
        }
    }
}

pub fn get_post(pool: Pool<ConnectionManager<PgConnection>>, post_id: Uuid) -> Option<Post> {
    info!("Get post with id: {}", post_id);
    let connection: &mut PgConnection = &mut pool.get().unwrap();

    let result = posts
        .filter(id.eq(post_id))
        .select(Post::as_select())
        .first(connection)
        .optional();

    match result {
        Ok(result) => result,
        Err(_) => return None,
    }
}

pub fn create_post(pool: Pool<ConnectionManager<PgConnection>>, create_post: CreatePost) -> Option<Post> {
    info!("Create post: {:?}", create_post);
    let connection: &mut PgConnection = &mut pool.get().unwrap();

    let new_post: Post = Post {
        id: Uuid::new_v4(),
        brand: create_post.brand,
        model: create_post.model,
        version: create_post.version,
        engine: create_post.engine,
        transmission: create_post.transmission,
        year: create_post.year,
        mileage: create_post.mileage,
        color: create_post.color,
        body: create_post.body,
        armored: create_post.armored,
        exchange: create_post.exchange,
        price: create_post.price,
        thumbnail_url: create_post.thumbnail_url,
        author: create_post.author,
        published: true,
    };

    let created_post = diesel::insert_into(posts)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(connection);

    match created_post {
        Ok(post) => Some(post),
        Err(err) => {
            error!("Unable to create post, error: {}", err.to_string());
            None
        }
    }
}

pub fn update_post(pool: Pool<ConnectionManager<PgConnection>>, post: Post) -> usize {
    info!("Update post to : {:?}", post);
    let connection: &mut PgConnection = &mut pool.get().unwrap();

    let update_count = diesel::update(posts)
        .filter(id.eq(post.id))
        .set(post)
        .execute(connection);

    update_count.unwrap_or_else(|_| 0)
}

pub fn delete_post(pool: Pool<ConnectionManager<PgConnection>>, post_id: Uuid) -> usize {
    info!("Delete post with id: {}", post_id);
    let connection: &mut PgConnection = &mut pool.get().unwrap();

    let delete_count = diesel::delete(posts)
        .filter(id.eq(post_id))
        .execute(connection);

    delete_count.unwrap_or_else(|_| 0)
}
