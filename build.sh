#!/usr/bin/bash

docker run \
    --rm \
    --mount type=bind,source="$(pwd)"/target,target=/output \
    "$(docker build -q .)"

