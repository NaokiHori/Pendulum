FROM rust:latest

RUN cargo install wasm-pack
RUN rustup component add clippy rustfmt

RUN useradd --create-home --uid "1000" --shell /bin/bash app
RUN mkdir -p /work /out /cache
RUN chown app:app /work /out /cache

USER app

ENV CARGO_HOME=/cache/cargo
ENV CARGO_TARGET_DIR=/cache/target

WORKDIR /work
