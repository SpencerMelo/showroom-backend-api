use diesel::{
    sql_types::{BigInt, Bool, Integer, Text},
    BoxableExpression,
};
use log::info;

use crate::schema::posts::{self, *};

pub enum PostColumn {
    Integer(Box<dyn BoxableExpression<posts::table, diesel::pg::Pg, SqlType = Integer>>),
    Text(Box<dyn BoxableExpression<posts::table, diesel::pg::Pg, SqlType = Text>>),
    Bool(Box<dyn BoxableExpression<posts::table, diesel::pg::Pg, SqlType = Bool>>),
    BigInteger(Box<dyn BoxableExpression<posts::table, diesel::pg::Pg, SqlType = BigInt>>),
}

pub fn get_column(sort_by: &str) -> PostColumn {
    match sort_by {
        "brand" => PostColumn::Text(Box::new(brand)),
        "model" => PostColumn::Text(Box::new(model)),
        "version" => PostColumn::Text(Box::new(version)),
        "engine" => PostColumn::Text(Box::new(engine)),
        "transmission" => PostColumn::Text(Box::new(transmission)),
        "year" => PostColumn::Integer(Box::new(year)),
        "mileage" => PostColumn::Integer(Box::new(mileage)),
        "color" => PostColumn::Text(Box::new(color)),
        "body" => PostColumn::Text(Box::new(body)),
        "armored" => PostColumn::Bool(Box::new(armored)),
        "exchange" => PostColumn::Bool(Box::new(exchange)),
        "price" => PostColumn::BigInteger(Box::new(price)),
        "thumbnail_url" => PostColumn::Text(Box::new(thumbnail_url)),
        "author" => PostColumn::Text(Box::new(author)),
        "published" => PostColumn::Bool(Box::new(published)),
        _ => {
            info!("Unknown column name: '{}', defaulting to 'model'", sort_by);
            PostColumn::Text(Box::new(model))
        }
    }
}
