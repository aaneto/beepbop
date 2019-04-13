# A Dockerfile that I made to run tests/bins on any computer with docker.
# Also useful for deploy strategies that uses containers.
FROM rust:1.34

WORKDIR /usr/src
RUN USER=root cargo new app

WORKDIR /usr/src/app

COPY Cargo.toml .

# This is meant to cache the build between
# source modifications.
RUN cargo build
# --release

COPY src/ src/

ENV API_KEY="YOUR_API_KEY"

RUN cargo test
# --release

# Run these if you use this dockerfile
# in a client code.
#
# RUN cargo install --path .
# CMD ["yourapp"]
