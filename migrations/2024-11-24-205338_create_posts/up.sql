CREATE TABLE posts
(
    id            UUID PRIMARY KEY,
    brand         VARCHAR NOT NULL,
    model         VARCHAR NOT NULL,
    version       VARCHAR NOT NULL,
    engine        VARCHAR NOT NULL,
    transmission  VARCHAR NOT NULL,
    year          INTEGER NOT NULL,
    mileage       INTEGER NOT NULL,
    color         VARCHAR NOT NULL,
    body          VARCHAR NOT NULL,
    armored       BOOLEAN NOT NULL,
    exchange      BOOLEAN NOT NULL,
    price         BIGINT NOT NULL,
    thumbnail_url VARCHAR NOT NULL,
    author        VARCHAR NOT NULL,
    published     BOOLEAN NOT NULL DEFAULT FALSE
);
