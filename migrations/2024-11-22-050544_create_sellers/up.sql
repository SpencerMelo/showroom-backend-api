CREATE TABLE sellers
(
    id            UUID PRIMARY KEY,
    name          VARCHAR NOT NULL,
    phone         VARCHAR NOT NULL,
    email         VARCHAR NOT NULL,

    addr_street   VARCHAR NOT NULL,
    addr_district VARCHAR NOT NULL,
    addr_city     VARCHAR NOT NULL,
    addr_state    VARCHAR NOT NULL,
    addr_zip_code INTEGER NOT NULL,

    start_hour    TIMESTAMPTZ,
    end_hour      TIMESTAMPTZ,

    -- Metadata
    created_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    deleted_at    TIMESTAMPTZ DEFAULT NULL,

    created_by    VARCHAR NOT NULL,
    updated_by    VARCHAR DEFAULT NULL,
    deleted_by    VARCHAR DEFAULT NULL
);
