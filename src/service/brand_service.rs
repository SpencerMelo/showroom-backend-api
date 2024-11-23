use chrono::Utc;

use crate::models::brand_models::{Brand, CreateBrand};
use crate::schema::brands::{self, dsl::*, BoxedQuery};
use crate::utils::brand_columns::{get_column, BrandColumn};

use diesel::pg::Pg;
use diesel::query_builder::QueryFragment;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{
    AppearsOnTable, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use log::{error, info};
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
        created_by: String::from("admin"),
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
