#!/bin/sh

docker run \
    -e "DATABASE_URL=postgres://postgres:admin@host.docker.internal/core" \
    -e "DATABASE_CONNECTION_TIMEOUT=10" \
    -e "APP_ENV=production" \
    -e "APP_HOST=0.0.0.0" \
    -e "APP_HOST=3000" \
    -e "RUST_LOG=info" \
    -p "55000:3000" \
    showroom-api
