use chrono::{DateTime, Utc};

use crate::models::brand_models::{Brand, CreateBrand, UpdateBrand};
use crate::schema::brands::{self, dsl::*, BoxedQuery};
use crate::utils::brand_columns::{get_column, BrandColumn};

use diesel::pg::Pg;
use diesel::query_builder::QueryFragment;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{
    AppearsOnTable, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use log::{error, info, warn};
use std::error::Error;
use uuid::Uuid;

pub fn get_brands(
    pool: Pool<ConnectionManager<PgConnection>>,
    offset: u32,
    limit: u32,
    sort_by: String,
    sort_order: String,
    filter_by: String,
    filter_term: String,
) -> Result<Vec<Brand>, Box<dyn Error>> {
    info!(
        "Get all brands starting at '{}', limited to '{}', sort by '{}' order '{}', filter by '{}' term '{}'",
        offset, limit, sort_by, sort_order, filter_by, filter_term
    );

    let mut query = brands::table
        .into_boxed()
        .limit(limit as i64)
        .offset(offset as i64);

    let sort_column: BrandColumn = get_column(sort_by.as_str());
    query = match sort_column {
        BrandColumn::Text(column) => sort_by_column(query, column, Some(sort_order)),
        BrandColumn::NullableText(column) => sort_by_column(query, column, Some(sort_order)),
    };

    if !filter_by.is_empty() && !filter_term.is_empty() {
        let filter_column: BrandColumn = get_column(filter_by.as_str());
        query = match filter_column {
            BrandColumn::Text(column) => query.filter(column.eq(filter_term)),
            BrandColumn::NullableText(column) => query.filter(column.eq(filter_term)),
        };
    }

    let brand_list = query.load(&mut get_connection(&pool)?);

    match brand_list {
        Ok(brand_list) => Ok(brand_list),
        Err(err) => {
            error!("Unable to retrieve brands, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn get_brand(
    pool: Pool<ConnectionManager<PgConnection>>,
    brand_id: Uuid,
) -> Result<Brand, Box<dyn Error>> {
    info!("Get brand with id: {}", brand_id);

    let result = brands::table
        .select(Brand::as_select())
        .filter(id.eq(brand_id))
        .first(&mut get_connection(&pool)?);

    match result {
        Ok(result) => Ok(result),
        Err(err) => {
            error!("Unable to retrieve brand, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn create_brand(
    pool: Pool<ConnectionManager<PgConnection>>,
    create_brand: CreateBrand,
) -> Result<Brand, Box<dyn Error>> {
    info!("Create brand: {:?}", create_brand);

    let new_brand: Brand = Brand {
        id: Uuid::new_v4(),
        name: create_brand.name,
        image_url: create_brand.image_url,
        thumbnail_url: create_brand.thumbnail_url,
        created_at: Utc::now(),
        updated_at: None,
        deleted_at: None,
        created_by: String::from("admin"), // TODO get it from request.
        updated_by: None,
        deleted_by: None,
    };

    let create_brand = diesel::insert_into(brands)
        .values(&new_brand)
        .returning(Brand::as_returning())
        .get_result(&mut get_connection(&pool)?);

    match create_brand {
        Ok(brand) => Ok(brand),
        Err(err) => {
            error!("Unable to create brand, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn create_brands(
    pool: Pool<ConnectionManager<PgConnection>>,
    new_brands: Vec<CreateBrand>,
) -> Result<Vec<Brand>, Box<dyn Error>> {
    info!("Create brands: {:?}", new_brands);

    if new_brands.is_empty() {
        warn!("No brands to create");
        return Err("No brands to create".into());
    }

    let mut brand_entities: Vec<Brand> = Vec::new();
    let now: DateTime<Utc> = Utc::now();
    let default_created_by = String::from("admin");

    for new_brand in new_brands {
        brand_entities.push(Brand {
            id: Uuid::new_v4(),
            name: new_brand.name,
            image_url: new_brand.image_url,
            thumbnail_url: new_brand.thumbnail_url,
            created_at: now,
            updated_at: None,
            deleted_at: None,
            created_by: default_created_by.clone(),
            updated_by: None,
            deleted_by: None,
        });
    }

    let result = diesel::insert_into(brands)
        .values(&brand_entities)
        .returning(Brand::as_returning())
        .get_results(&mut get_connection(&pool)?);

    match result {
        Ok(created) => Ok(created),
        Err(err) => {
            error!("Unable to create brands, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn update_brand(
    pool: Pool<ConnectionManager<PgConnection>>,
    brand_id: Uuid,
    updated_brand: UpdateBrand,
) -> Result<usize, Box<dyn Error>> {
    info!("Update brand {} to {:?}", brand_id, updated_brand);

    let update_count = diesel::update(brands)
        .filter(id.eq(brand_id))
        .set((
            updated_brand,
            updated_at.eq(Utc::now()),
            updated_by.eq(String::from("admin")),
        ))
        .execute(&mut get_connection(&pool)?);

    match update_count {
        Ok(count) => Ok(count),
        Err(err) => {
            error!("Unable to update brands, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn delete_brand(
    pool: Pool<ConnectionManager<PgConnection>>,
    brand_id: Uuid,
) -> Result<usize, Box<dyn Error>> {
    info!("Delete brand with id: {}", brand_id);

    let delete_count = diesel::delete(brands)
        .filter(id.eq(brand_id))
        .execute(&mut get_connection(&pool)?);

    match delete_count {
        Ok(count) => Ok(count),
        Err(err) => {
            error!("Unable to delete brand, error: {}", err);
            Err(err.into())
        }
    }
}

pub fn delete_brands(
    pool: Pool<ConnectionManager<PgConnection>>,
    brands_ids: Vec<Uuid>,
) -> Result<usize, Box<dyn Error>> {
    info!("Delete brands with ids: {:?}", brands_ids);

    let delete_count = diesel::delete(brands)
        .filter(id.eq_any(brands_ids))
        .execute(&mut get_connection(&pool)?);

    match delete_count {
        Ok(count) => {
            info!("Posts delete count: {}", count);
            Ok(count)
        }
        Err(err) => {
            error!("Unable to delete brands, error: {}", err);
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
    U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<brands::table>,
{
    match sort_dir.as_ref().map(String::as_str) {
        Some("asc") => query.order_by(column.asc()),
        Some("desc") => query.order_by(column.desc()),
        _ => query,
    }
}
