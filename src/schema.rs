// @generated automatically by Diesel CLI.

diesel::table! {
    brands (id) {
        id -> Uuid,
        name -> Varchar,
        image_url -> Varchar,
        thumbnail_url -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by -> Varchar,
        updated_by -> Nullable<Varchar>,
        deleted_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    cars (id) {
        id -> Uuid,
        version_id -> Uuid,
        mileage -> Int4,
        color -> Varchar,
        armored -> Bool,
        owner -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by -> Varchar,
        updated_by -> Nullable<Varchar>,
        deleted_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    listings (id) {
        id -> Uuid,
        car_id -> Uuid,
        seller_id -> Uuid,
        price -> Int4,
        exchange -> Bool,
        phone -> Varchar,
        email -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by -> Varchar,
        updated_by -> Nullable<Varchar>,
        deleted_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    models (id) {
        id -> Uuid,
        brand_id -> Uuid,
        name -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by -> Varchar,
        updated_by -> Nullable<Varchar>,
        deleted_by -> Nullable<Varchar>,
    }
}

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

diesel::table! {
    sellers (id) {
        id -> Uuid,
        name -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        addr_street -> Varchar,
        addr_district -> Varchar,
        addr_city -> Varchar,
        addr_state -> Varchar,
        addr_zip_code -> Int4,
        start_hour -> Nullable<Timestamptz>,
        end_hour -> Nullable<Timestamptz>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by -> Varchar,
        updated_by -> Nullable<Varchar>,
        deleted_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    versions (id) {
        id -> Uuid,
        model_id -> Uuid,
        name -> Varchar,
        engine -> Varchar,
        transmission -> Varchar,
        year -> Int4,
        body -> Varchar,
        doors -> Int4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        created_by -> Varchar,
        updated_by -> Nullable<Varchar>,
        deleted_by -> Nullable<Varchar>,
    }
}

diesel::joinable!(cars -> versions (version_id));
diesel::joinable!(listings -> cars (car_id));
diesel::joinable!(listings -> sellers (seller_id));
diesel::joinable!(models -> brands (brand_id));
diesel::joinable!(versions -> models (model_id));

diesel::allow_tables_to_appear_in_same_query!(
    brands,
    cars,
    listings,
    models,
    posts,
    sellers,
    versions,
);
