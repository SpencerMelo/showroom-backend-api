use diesel::pg::Pg;
use diesel::query_builder::QueryFragment;
use diesel::sql_types::{Bool, Double, Integer, Text};
use showroom_api::models::models::{CreatePost, Post};
use showroom_api::schema::posts::{self, brand, dsl::*, model, BoxedQuery};

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{
    AppearsOnTable, BoxableExpression, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use log::{error, info};
use std::error::Error;

use uuid::Uuid;

pub fn get_posts(
    pool: Pool<ConnectionManager<PgConnection>>,
    offset: u32,
    limit: u32,
    sort_by: String,
    sort_order: String,
) -> Result<Vec<Post>, Box<dyn Error>> {
    info!(
        "Get all posts from page {}, limited to {}, sort by {}, order {}",
        offset, limit, sort_by, sort_order
    );

    let mut query = posts::table
        .into_boxed()
        .filter(published.eq(true))
        .limit(limit as i64)
        .offset(offset as i64);

    let column: Column = get_column(sort_by.as_str());

    query = match column {
        Column::Integer(col) => sort_by_column(query, col, Some(sort_order)),
        Column::Text(col) => sort_by_column(query, col, Some(sort_order)),
        Column::Double(col) => sort_by_column(query, col, Some(sort_order)),
        Column::Bool(col) => sort_by_column(query, col, Some(sort_order)),
    };

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
    pool: Pool<ConnectionManager<PgConnection>>,
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
    pool: Pool<ConnectionManager<PgConnection>>,
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

pub fn update_post(
    pool: Pool<ConnectionManager<PgConnection>>,
    post: Post,
) -> Result<usize, Box<dyn Error>> {
    info!("Update post to : {:?}", post);

    let update_count = diesel::update(posts)
        .filter(id.eq(post.id))
        .set(post)
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
    pool: Pool<ConnectionManager<PgConnection>>,
    post_id: Uuid,
) -> Result<usize, Box<dyn Error>> {
    info!("Delete post with id: {}", post_id);

    let delete_count = diesel::delete(posts)
        .filter(id.eq(post_id))
        .execute(&mut get_connection(&pool)?);

    match delete_count {
        Ok(count) => Ok(count),
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

enum Column {
    Integer(Box<dyn BoxableExpression<posts::table, diesel::pg::Pg, SqlType = Integer>>),
    Text(Box<dyn BoxableExpression<posts::table, diesel::pg::Pg, SqlType = Text>>),
    Double(Box<dyn BoxableExpression<posts::table, diesel::pg::Pg, SqlType = Double>>),
    Bool(Box<dyn BoxableExpression<posts::table, diesel::pg::Pg, SqlType = Bool>>),
}

fn get_column(sort_by: &str) -> Column {
    match sort_by {
        "brand" => Column::Text(Box::new(brand)),
        "model" => Column::Text(Box::new(model)),
        "version" => Column::Text(Box::new(version)),
        "engine" => Column::Text(Box::new(engine)),
        "transmission" => Column::Text(Box::new(transmission)),
        "year" => Column::Text(Box::new(year)),
        "mileage" => Column::Integer(Box::new(mileage)),
        "color" => Column::Text(Box::new(color)),
        "body" => Column::Text(Box::new(body)),
        "armored" => Column::Bool(Box::new(armored)),
        "exchange" => Column::Bool(Box::new(exchange)),
        "price" => Column::Text(Box::new(price)),
        "thumbnail_url" => Column::Text(Box::new(thumbnail_url)),
        "author" => Column::Text(Box::new(author)),
        "published" => Column::Bool(Box::new(published)),
        _ => {
            info!("Unknown column name: {}, defaulting to 'model'", sort_by);
            Column::Text(Box::new(model))
        }
    }
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
