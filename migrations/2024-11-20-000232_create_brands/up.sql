CREATE TABLE brands
(
    id            UUID PRIMARY KEY,
    name          VARCHAR NOT NULL,
    image_url     VARCHAR NOT NULL,
    thumbnail_url VARCHAR NOT NULL,

    -- Metadata
    created_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    deleted_at    TIMESTAMPTZ DEFAULT NULL,

    created_by    VARCHAR NOT NULL,
    updated_by    VARCHAR DEFAULT NULL,
    deleted_by    VARCHAR DEFAULT NULL
);
