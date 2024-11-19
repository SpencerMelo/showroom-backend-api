// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Uuid,
        brand -> Varchar,
        model -> Varchar,
        version -> Varchar,
        engine -> Varchar,
        transmission -> Varchar,
        year -> Int4,
        mileage -> Int4,
        color -> Varchar,
        body -> Varchar,
        armored -> Bool,
        exchange -> Bool,
        price -> Int8,
        thumbnail_url -> Varchar,
        author -> Varchar,
        published -> Bool,
    }
}
