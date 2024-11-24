use diesel::pg::Pg;
use diesel::query_builder::QueryFragment;

use crate::models::post_models::{CreatePost, Post, UpdatePost};
use crate::schema::posts::{self, dsl::*, BoxedQuery};
use crate::utils::post_columns::{get_column, PostColumn};

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{
    AppearsOnTable, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use log::{error, info};
use std::error::Error;
use std::sync::Arc;

use uuid::Uuid;

pub fn get_posts(
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    offset: u32,
    limit: u32,
    sort_by: String,
    sort_order: String,
    filter_by: String,
    filter_term: String,
) -> Result<Vec<Post>, Box<dyn Error>> {
    info!(
        "Get all posts starting at '{}', limited to '{}', sort by '{}' order '{}', filter by '{}' term '{}'",
        offset, limit, sort_by, sort_order, filter_by, filter_term
    );

    let mut query = posts::table
        .into_boxed()
        .filter(published.eq(true))
        .limit(limit as i64)
        .offset(offset as i64);

    let sort_column: PostColumn = get_column(sort_by.as_str());
    query = match sort_column {
        PostColumn::Integer(column) => sort_by_column(query, column, Some(sort_order)),
        PostColumn::Text(column) => sort_by_column(query, column, Some(sort_order)),
        PostColumn::Bool(column) => sort_by_column(query, column, Some(sort_order)),
        PostColumn::BigInteger(column) => sort_by_column(query, column, Some(sort_order)),
    };

    if !filter_by.is_empty() && !filter_term.is_empty() {
        let filter_column: PostColumn = get_column(filter_by.as_str());
        query = match filter_column {
            PostColumn::Integer(column) => query.filter(column.eq(filter_term.parse::<i32>()?)),
            PostColumn::Text(column) => query.filter(column.eq(filter_term)),
            PostColumn::Bool(column) => query.filter(column.eq(filter_term.parse::<bool>()?)),
            PostColumn::BigInteger(column) => query.filter(column.eq(filter_term.parse::<i64>()?)),
        };
    }

    let post_list = query.load(&mut get_connection(&pool)?);

    match post_list {
        Ok(post_list) => Ok(post_list),
        Err(err) => {
            error!("Unable to retrieve posts, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn get_post(
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    post_id: Uuid,
) -> Result<Post, Box<dyn Error>> {
    info!("Get post with id: {}", post_id);

    let result = posts
        .filter(id.eq(post_id))
        .select(Post::as_select())
        .first(&mut get_connection(&pool)?);

    match result {
        Ok(result) => Ok(result),
        Err(err) => {
            error!("Unable to retrieve post, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn create_post(
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    create_post: CreatePost,
) -> Result<Post, Box<dyn Error>> {
    info!("Create post: {:?}", create_post);

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
        .get_result(&mut get_connection(&pool)?);

    match created_post {
        Ok(post) => Ok(post),
        Err(err) => {
            error!("Unable to create post, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn create_posts(
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    new_posts: Vec<CreatePost>,
) -> Result<Vec<Post>, Box<dyn Error>> {
    info!("Create posts: {:?}", new_posts);

    if new_posts.is_empty() {
        info!("No posts to create");
        return Err("No posts to create".into());
    }

    let mut post_entities: Vec<Post> = Vec::new();

    for new_post in new_posts {
        post_entities.push(Post {
            id: Uuid::new_v4(),
            brand: new_post.brand,
            model: new_post.model,
            version: new_post.version,
            engine: new_post.engine,
            transmission: new_post.transmission,
            year: new_post.year,
            mileage: new_post.mileage,
            color: new_post.color,
            body: new_post.body,
            armored: new_post.armored,
            exchange: new_post.exchange,
            price: new_post.price,
            thumbnail_url: new_post.thumbnail_url,
            author: new_post.author,
            published: true,
        });
    }

    let result = diesel::insert_into(posts)
        .values(&post_entities)
        .returning(Post::as_returning())
        .get_results(&mut get_connection(&pool)?);

    match result {
        Ok(created) => Ok(created),
        Err(err) => {
            error!("Unable to create posts, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn update_post(
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    post_id: Uuid,
    updated_post: UpdatePost,
) -> Result<usize, Box<dyn Error>> {
    info!("Update post {} to {:?}", post_id, updated_post);

    let updated_post: Post = Post {
        id: post_id,
        brand: updated_post.brand,
        model: updated_post.model,
        version: updated_post.version,
        engine: updated_post.engine,
        transmission: updated_post.transmission,
        year: updated_post.year,
        mileage: updated_post.mileage,
        color: updated_post.color,
        body: updated_post.body,
        armored: updated_post.armored,
        exchange: updated_post.exchange,
        price: updated_post.price,
        thumbnail_url: updated_post.thumbnail_url,
        author: updated_post.author,
        published: updated_post.published,
    };

    let update_count = diesel::update(posts)
        .filter(id.eq(post_id))
        .set(updated_post)
        .execute(&mut get_connection(&pool)?);

    match update_count {
        Ok(count) => Ok(count),
        Err(err) => {
            error!("Unable to update posts, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn delete_post(
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    post_id: Uuid,
) -> Result<usize, Box<dyn Error>> {
    info!("Delete post with id: {}", post_id);

    let delete_count = diesel::delete(posts)
        .filter(id.eq(post_id))
        .execute(&mut get_connection(&pool)?);

    match delete_count {
        Ok(count) => Ok(count),
        Err(err) => {
            error!("Unable to delete post, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn delete_posts(
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    post_ids: Vec<Uuid>,
) -> Result<usize, Box<dyn Error>> {
    info!("Delete posts with ids: {:?}", post_ids);

    let delete_count = diesel::delete(posts)
        .filter(id.eq_any(post_ids))
        .execute(&mut get_connection(&pool)?);

    match delete_count {
        Ok(count) => {
            info!("Posts delete count: {}", count);
            Ok(count)
        }
        Err(err) => {
            error!("Unable to delete posts, error: {}", err);
            Err(err.into())
        }
    }
}

fn get_connection(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Box<dyn Error>> {
    let connection = pool.get().map_err(|err| {
        error!("Unable to connect to database, error: {}", err);
        Box::new(err) as Box<dyn Error>
    })?;

    Ok(connection)
}

// https://stackoverflow.com/a/62029781
fn sort_by_column<U: 'static + std::marker::Send>(
    query: BoxedQuery<'static, Pg>,
    column: U,
    sort_dir: Option<String>,
) -> BoxedQuery<'static, Pg>
where
    U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<posts::table>,
{
    match sort_dir.as_ref().map(String::as_str) {
        Some("asc") => query.order_by(column.asc()),
        Some("desc") => query.order_by(column.desc()),
        _ => query,
    }
}
