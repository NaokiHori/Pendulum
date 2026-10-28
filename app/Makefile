IMAGE := naokihori/pendulum-app:latest
CONTAINER := pendulum-app

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
    wasm-pack build --target web --out-dir /app/pkg --features library-crate
	docker cp $(CONTAINER):/app/pkg ./pkg
	docker rm $(CONTAINER)

build-image:
	docker build --progress=plain -t $(IMAGE) .

run:
	docker run \
    --init \
    --rm \
    -v "${PWD}/Cargo.toml":/app/Cargo.toml:ro \
    -v "${PWD}/Cargo.lock":/app/Cargo.lock:ro \
    -v "${PWD}/src":/app/src:ro \
    $(IMAGE) \
    cargo run --locked

fmt:
	docker run \
    --init \
    --rm \
    -v "${PWD}/Cargo.toml":/app/Cargo.toml:ro \
    -v "${PWD}/Cargo.lock":/app/Cargo.lock:ro \
    -v "${PWD}/src":/app/src \
    $(IMAGE) \
    cargo fmt

clippy:
	docker run \
    --init \
    --rm \
    -v "${PWD}/Cargo.toml":/app/Cargo.toml:ro \
    -v "${PWD}/Cargo.lock":/app/Cargo.lock:ro \
    -v "${PWD}/src:/app/src:ro" \
    $(IMAGE) \
    cargo clippy

