use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = crate::schema::post)]
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