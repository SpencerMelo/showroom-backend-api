CREATE TABLE cars
(
    id            UUID PRIMARY KEY,
    version_id    UUID NOT NULL,
    mileage       INTEGER NOT NULL,
    color         VARCHAR NOT NULL,
    armored       BOOLEAN NOT NULL,
    owner         VARCHAR NOT NULL,

    -- Metadata
    created_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    deleted_at    TIMESTAMPTZ DEFAULT NULL,

    created_by    VARCHAR NOT NULL,
    updated_by    VARCHAR DEFAULT NULL,
    deleted_by    VARCHAR DEFAULT NULL,

    -- Foreign keys
    FOREIGN KEY (version_id) REFERENCES versions(id)
);
