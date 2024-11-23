use diesel::{
    pg::Pg,
    sql_types::{Nullable, Text},
    BoxableExpression,
};
use log::info;

use crate::schema::brands::{self, *};

pub enum BrandColumn {
    Text(Box<dyn BoxableExpression<brands::table, Pg, SqlType = Text>>),
    NullableText(Box<dyn BoxableExpression<brands::table, Pg, SqlType = Nullable<Text>>>),
}

pub fn get_column(sort_by: &str) -> BrandColumn {
    match sort_by {
        "name" => BrandColumn::Text(Box::new(name)),
        "created_by" => BrandColumn::Text(Box::new(created_by)),
        "updated_by" => BrandColumn::NullableText(Box::new(updated_by)),
        _ => {
            info!("Unknown column name: '{}', defaulting to 'name'", sort_by);
            BrandColumn::Text(Box::new(name))
        }
    }
}
