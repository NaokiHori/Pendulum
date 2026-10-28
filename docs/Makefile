IMAGE := naokihori/pendulum-docs:latest
CONTAINER := pendulum-docs

help:
	@echo "build       : build artifacts"
	@echo "build-image : build docker image"

build:
	docker rm -f $(CONTAINER) 2>/dev/null || true
	docker run \
    --name $(CONTAINER) \
    -v "${PWD}/source":/app/source:ro \
    $(IMAGE) \
    python -m sphinx -b html /app/source /app/build
	docker cp $(CONTAINER):/app/build ./build
	docker rm $(CONTAINER)

build-image:
	docker build --progress=plain -t $(IMAGE) .

