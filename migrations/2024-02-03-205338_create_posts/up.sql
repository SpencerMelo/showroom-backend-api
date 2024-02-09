CREATE TABLE posts
(
    id            uuid primary key,
    brand         varchar not null,
    model         varchar not null,
    version       varchar not null,
    engine        varchar not null,
    transmission  varchar not null,
    year          varchar not null,
    mileage       integer not null,
    color         varchar not null,
    body          varchar not null,
    armored       boolean not null,
    exchange      boolean not null,
    price         varchar not null,
    thumbnail_url varchar not null,
    author        varchar not null,
    published     boolean not null default false
);
