################################
# Build the source code
################################
FROM rust:1.40 as build

WORKDIR sdk
COPY . .
RUN cargo build --release && strip /sdk/target/release/bash-sdk


################################
# Copy the bash sdk to the base image
################################
FROM registry.cto.ai/official_images/base:2-buster-slim

RUN apt-get update && \
    apt-get install -y curl

COPY --from=build /sdk/target/release/bash-sdk /usr/local/bin/ctoai
