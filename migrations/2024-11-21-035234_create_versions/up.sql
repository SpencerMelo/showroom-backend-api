CREATE TABLE versions
(
    id            UUID PRIMARY KEY,
    model_id      UUID NOT NULL,
    name          VARCHAR NOT NULL,
    engine        VARCHAR NOT NULL,
    transmission  VARCHAR NOT NULL,
    year          INTEGER NOT NULL,
    body          VARCHAR NOT NULL,
    doors         INTEGER NOT NULL,

    -- Metadata
    created_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    deleted_at    TIMESTAMPTZ DEFAULT NULL,

    created_by    VARCHAR NOT NULL,
    updated_by    VARCHAR DEFAULT NULL,
    deleted_by    VARCHAR DEFAULT NULL,

    -- Foreign keys
    FOREIGN KEY (model_id) REFERENCES models(id)
);
