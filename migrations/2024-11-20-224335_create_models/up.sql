CREATE TABLE models
(
    id            UUID PRIMARY KEY,
    brand_id      UUID NOT NULL,
    name          VARCHAR NOT NULL,

    -- Metadata
    created_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    deleted_at    TIMESTAMPTZ DEFAULT NULL,

    created_by    VARCHAR NOT NULL,
    updated_by    VARCHAR DEFAULT NULL,
    deleted_by    VARCHAR DEFAULT NULL,

    -- Foreign keys
    FOREIGN KEY (brand_id) REFERENCES brands(id)
);
