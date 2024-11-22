CREATE TABLE listings
(
    id            UUID PRIMARY KEY,
    car_id        UUID NOT NULL,
    seller_id     UUID NOT NULL,
    price         INTEGER NOT NULL,
    exchange      BOOLEAN NOT NULL,
    phone         VARCHAR NOT NULL,
    email         VARCHAR NOT NULL,

    -- Metadata
    created_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at    TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    deleted_at    TIMESTAMPTZ DEFAULT NULL,

    created_by    VARCHAR NOT NULL,
    updated_by    VARCHAR DEFAULT NULL,
    deleted_by    VARCHAR DEFAULT NULL,

    -- Foreign keys
    FOREIGN KEY (car_id) REFERENCES cars(id),
    FOREIGN KEY (seller_id) REFERENCES sellers(id)
);
