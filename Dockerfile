FROM rust:1.82-bullseye AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && \
    apt-get install -y libpq5 && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/showroom-api /usr/local/bin/showroom-api

EXPOSE 3000

CMD [ "showroom-api" ]
