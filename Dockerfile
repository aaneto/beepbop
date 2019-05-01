# Docker file for testing purposes.
FROM rust:1.34 as cargo-build

WORKDIR /usr/src
RUN USER=root cargo new app

WORKDIR /usr/src/app

RUN mkdir /usr/src/app/optional_builder
COPY optional_builder /usr/src/app/optional_builder

COPY Cargo.toml .

# This is meant to cache the build between
# source modifications.
RUN cargo build

COPY src/ src/

ENV API_KEY="YOUR_API_KEY"
ENV CHAT_ID="YOUR_CHAT_ID"

# A valid message on CHAT_ID that can be forwarded.
ENV MESSAGE_ID="MESSAGE_ID"

RUN mkdir res
COPY res/ res/

CMD ["cargo", "test"]
