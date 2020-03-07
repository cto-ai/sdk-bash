################################
# Build the source code
################################
FROM rust:1.40 as build

WORKDIR sdk
COPY . .
RUN cargo build --release


################################
# Copy the bash sdk to the base image
################################
FROM registry.cto.ai/official_images/base:buster-slim

RUN apt-get update && \
    apt-get install -y curl

COPY --from=build /sdk/target/release/bash-sdk /usr/local/bin/ctoai
