FROM rust:alpine

RUN rustup component add clippy rustfmt \
  && rustup target add wasm32-unknown-unknown

RUN adduser -D -u 1000 app

USER app
WORKDIR /app

ENV CARGO_HOME=/app/.cargo \
  CARGO_TARGET_DIR=/app/target

RUN mkdir "$CARGO_HOME" "$CARGO_TARGET_DIR"
