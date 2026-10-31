IMAGE := naokihori/pendulum-app:latest
CONTAINER := pendulum-app
FEATURES ?= implicit

help:
	@echo "build  : build wasm package"
	@echo "run    : execute pendulum simulator"
	@echo "fmt    : run fmt"
	@echo "clippy : run clippy"

build:
	docker rm -f $(CONTAINER) 2>/dev/null || true
	docker run \
    --name $(CONTAINER) \
    -v "${PWD}/Cargo.toml":/app/Cargo.toml:ro \
    -v "${PWD}/Cargo.lock":/app/Cargo.lock:ro \
    -v "${PWD}/src":/app/src:ro \
    $(IMAGE) \
    cargo build --lib --target wasm32-unknown-unknown --release
	docker cp $(CONTAINER):/app/target/wasm32-unknown-unknown/release/pendulum.wasm .
	docker rm $(CONTAINER)

build-image:
	docker build --progress=plain -t $(IMAGE) .

run:
	mkdir -p output
	docker run \
    --init \
    --rm \
    -v "${PWD}/Cargo.toml":/app/Cargo.toml:ro \
    -v "${PWD}/Cargo.lock":/app/Cargo.lock:ro \
    -v "${PWD}/src":/app/src:ro \
    -v "${PWD}/output":/app/output \
    $(IMAGE) \
    cargo run --locked --release --features=$(FEATURES)

fmt:
	docker run \
    --rm \
    -v "${PWD}/Cargo.toml":/app/Cargo.toml:ro \
    -v "${PWD}/Cargo.lock":/app/Cargo.lock:ro \
    -v "${PWD}/src":/app/src \
    $(IMAGE) \
    cargo fmt

clippy:
	docker run \
    --rm \
    -v "${PWD}/Cargo.toml":/app/Cargo.toml:ro \
    -v "${PWD}/Cargo.lock":/app/Cargo.lock:ro \
    -v "${PWD}/src:/app/src:ro" \
    $(IMAGE) \
    cargo clippy

