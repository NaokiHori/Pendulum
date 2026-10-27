IMAGE := naokihori/pendulum-app:latest
CONTAINER := pendulum-app

help:
	@echo "build  : build wasm package"
	@echo "run    : execute pendulum simulator"
	@echo "fmt    : run fmt"
	@echo "clippy : run clippy"

build:
	docker rm -f $(CONTAINER) 2>/dev/null || true
	docker run --name $(CONTAINER) -v ${PWD}:/work:ro $(IMAGE) wasm-pack build --target web --out-dir /out/pkg --features library-crate
	docker cp $(CONTAINER):/out/pkg ./pkg
	docker rm $(CONTAINER)

run:
	docker run --init --rm -v ${PWD}:/work $(IMAGE) cargo run --locked

fmt:
	docker run --init --rm -v ${PWD}:/work $(IMAGE) cargo fmt

clippy:
	docker run --init --rm -v ${PWD}:/work $(IMAGE) cargo clippy

