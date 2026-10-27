IMAGE := naokihori/pendulum-web:latest
CONTAINER := pendulum-web

help:
	@echo "build : build artifacts"
	@echo "dev   : launch development server"

build:
	docker rm -f $(CONTAINER) 2>/dev/null || true
	docker run \
    --name $(CONTAINER) \
    -v ${PWD}/index.html:/app/index.html:ro \
    -v ${PWD}/style.css:/app/style.css:ro \
    -v ${PWD}/pkg:/app/pkg:ro \
    -v ${PWD}/src:/app/src:ro \
    $(IMAGE) \
    npx vite build --outDir=/app/dist --base=https://naokihori.github.io/Pendulum/
	docker cp $(CONTAINER):/app/dist ./dist
	docker rm $(CONTAINER)

dev:
	docker run \
    --rm \
    -p 5173:5173 \
    --name $(CONTAINER) \
    -v ${PWD}/index.html:/app/index.html:ro \
    -v ${PWD}/style.css:/app/style.css:ro \
    -v ${PWD}/pkg:/app/pkg:ro \
    -v ${PWD}/src:/app/src:ro \
    $(IMAGE) \
    npx vite dev --host

