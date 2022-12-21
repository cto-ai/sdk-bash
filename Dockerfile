################################
#   ____  _   _ ___ _     ____
#  | __ )| | | |_ _| |   |  _ \
#  |  _ \| | | || || |   | | | |
#  | |_) | |_| || || |___| |_| |
#  |____/ \___/|___|_____|____/
#
# Builder layer
FROM rust:1.62 AS build

## arm64 tweaks
# context: https://github.com/docker/buildx/issues/359#issuecomment-1331443419
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

WORKDIR sdk
COPY . .

RUN cargo build --release \
    && strip /sdk/target/release/bash-sdk /sdk/target/release/sdk /sdk/target/release/ux \
    && mv /sdk/target/release/bash-sdk /sdk/target/release/ctoai

########################
#   ____ ___ ____ _____
#  |  _ \_ _/ ___|_   _|
#  | | | | |\___ \ | |
#  | |_| | | ___) || |
#  |____/___|____/ |_|
#
# Binary distribution layer
FROM scratch AS dist

COPY --from=build /sdk/target/release/ctoai /sdk/target/release/sdk /sdk/target/release/ux /

#########################
#   ____  _______     __
#  |  _ \| ____\ \   / /
#  | | | |  _|  \ \ / /
#  | |_| | |___  \ V /
#  |____/|_____|  \_/
#
# SDK-bash image is no longer built from this repository,
# however this stage could be used for dev purposes
FROM debian:bullseye-slim

RUN apt-get update \
    && apt-get install -y curl

COPY --from=build /sdk/target/release/ctoai /sdk/target/release/sdk /sdk/target/release/ux /usr/local/bin/
