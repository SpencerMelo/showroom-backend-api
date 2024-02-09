use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Insertable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: Uuid,
    pub brand: String,
    pub model: String,
    pub version: String,
    pub engine: String,
    pub transmission: String,
    pub year: String,
    pub mileage: i32,
    pub color: String,
    pub body: String,
    pub armored: bool,
    pub exchange: bool,
    pub price: String,
    pub thumbnail_url: String,
    pub author: String,
    pub published: bool
}

#[derive(serde::Deserialize)]
pub struct CreatePost {
    pub brand: String,
    pub model: String,
    pub version: String,
    pub engine: String,
    pub transmission: String,
    pub year: String,
    pub mileage: i32,
    pub color: String,
    pub body: String,
    pub armored: bool,
    pub exchange: bool,
    pub price: String ,
    pub thumbnail_url: String,
    pub author: String
}