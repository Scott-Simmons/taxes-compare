IMAGE_NAME := taxes-compare-webserver
APP_NAME := taxes-compare
USERNAME := simmonsd99
VERSION := $(shell cat VERSION)
WEBSERVER_PORT := 8080
HOST := localhost:3000
PROTOCOL := http


# For local dev without containerisation (will use default env vars set up to be local)
local: install-local build-local run-local

install-local:
	npm install

build-local:
	npm run build

run-local:
	serve -s build -l $(WEBSERVER_PORT)

clean-all:
	rm -rf node_modules
	rm -rf build
	rm -rf package-lock.json

clean:
	rm -rf node_modules
	rm -rf build

lint:
	npm run lint


# For containerisation (env vars are injected at runtime)
docker: build-docker run-docker

build-docker:
	docker build \
		--no-cache \
		-t $(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION) \
		.

run-docker:
	docker run \
		-it \
		--rm \
		-p $(WEBSERVER_PORT):$(WEBSERVER_PORT) \
		-e REACT_APP_BACKEND_PROTOCOL=$(PROTOCOL) \
		-e REACT_APP_BACKEND_HOST=$(HOST) \
		-e NGINX_PORT=$(WEBSERVER_PORT) \
	$(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION)

publish-docker:
	docker push \
		$(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION)

.PHONY: build
