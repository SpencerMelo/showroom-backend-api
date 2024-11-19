use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Insertable, Identifiable, AsChangeset, Debug,
)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: Uuid,
    pub brand: String,
    pub model: String,
    pub version: String,
    pub engine: String,
    pub transmission: String,
    pub year: i32,
    pub mileage: i32,
    pub color: String,
    pub body: String,
    pub armored: bool,
    pub exchange: bool,
    pub price: i64,
    pub thumbnail_url: String,
    pub author: String,
    pub published: bool,
}

#[derive(serde::Deserialize, Debug)]
pub struct CreatePost {
    pub brand: String,
    pub model: String,
    pub version: String,
    pub engine: String,
    pub transmission: String,
    pub year: i32,
    pub mileage: i32,
    pub color: String,
    pub body: String,
    pub armored: bool,
    pub exchange: bool,
    pub price: i64,
    pub thumbnail_url: String,
    pub author: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdatePost {
    pub brand: String,
    pub model: String,
    pub version: String,
    pub engine: String,
    pub transmission: String,
    pub year: i32,
    pub mileage: i32,
    pub color: String,
    pub body: String,
    pub armored: bool,
    pub exchange: bool,
    pub price: i64,
    pub thumbnail_url: String,
    pub author: String,
    pub published: bool,
}
