#########################
#   ____  _______     __
#  |  _ \| ____\ \   / /
#  | | | |  _|  \ \ / /
#  | |_| | |___  \ V /
#  |____/|_____|  \_/
#
# CTO.ai SDK-bash image is no longer built from this Dockerfile,
# However feel free to use it for dev purposes

# Builder layer
FROM rust:1.72 AS build

## arm64 tweaks
# context: https://github.com/docker/buildx/issues/359#issuecomment-1331443419
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

WORKDIR sdk
COPY . .

RUN cargo build --release \
    && strip /sdk/target/release/ctoai /sdk/target/release/sdk /sdk/target/release/ux

# Binary distribution layer
FROM scratch AS dist

COPY --from=build /sdk/target/release/ctoai /sdk/target/release/sdk /sdk/target/release/ux /

FROM debian:bullseye-slim

RUN apt-get update \
    && apt-get install -y curl

COPY --from=build /sdk/target/release/ctoai /sdk/target/release/sdk /sdk/target/release/ux /usr/local/bin/
