// @generated automatically by Diesel CLI.

diesel::table! {
    post (id) {
        id -> Uuid,
        brand -> Varchar,
        model -> Varchar,
        version -> Varchar,
        engine -> Varchar,
        transmission -> Varchar,
        year -> Varchar,
        mileage -> Int4,
        color -> Varchar,
        body -> Varchar,
        armored -> Bool,
        exchange -> Bool,
        price -> Varchar,
        thumbnail_url -> Varchar,
        author -> Varchar,
        published -> Bool,
    }
}
