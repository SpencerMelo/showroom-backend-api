use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Queryable, Selectable, Serialize, Deserialize, Insertable, Identifiable, AsChangeset, Debug,
)]
#[diesel(table_name = crate::schema::brands)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
    pub image_url: String,
    pub thumbnail_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_by: String,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CreateBrand {
    pub name: String,
    pub image_url: String,
    pub thumbnail_url: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdateBrand {
    pub name: String,
    pub image_url: String,
    pub thumbnail_url: String,
}
