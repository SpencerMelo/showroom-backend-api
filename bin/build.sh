#!/bin/sh

diesel migration run

docker build -t showroom-api:latest .
