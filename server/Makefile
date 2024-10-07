BINARY_NAME := taxes-compare
IMAGE_NAME := taxes-compare
APP_NAME := taxes-compare
USERNAME := simmonsd99
VERSION := $(shell cat VERSION)
SERVER_PORT := 3001

local: build-local run-local

DRY_RUN ?= false
build-local:
		 cargo update;
		 @if [ $(DRY_RUN) = true ]; then \
						 echo dry run...; \
						 cargo check; \
		 else \
						 cargo build; \
		 fi

# can also do `cargo run`
BACKTRACE_CMD ?= RUST_LOG=DEBUG
run-local:
	SERVER_PORT=$(SERVER_PORT) $(BACKTRACE_CMD) ./target/debug/$(BINARY_NAME)

lint:
	cargo fmt

clean:
	cargo clean

test-local:
	cargo test -- --nocapture

docker: build-docker run-docker
run-docker:
	docker run \
		--rm \
		-p $(SERVER_PORT):$(SERVER_PORT) \
		-e SERVER_PORT=$(SERVER_PORT) \
		$(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION)

publish-docker:
	docker push \
		$(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION)

build-docker:
	docker build \
		-t $(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION) \
		.
