BINARY_NAME := taxes-compare
IMAGE_NAME := taxes-compare
APP_NAME := taxes-compare
USERNAME := simmonsd99
VERSION := 0.0.2

build:
	docker build . -t $(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION)

DRY_RUN ?= false
build-local:
		 cargo update;
		 @if [ $(DRY_RUN) = true ]; then \
						 echo dry run...; \
						 cargo check; \
		 else \
						 cargo build; \
		 fi

run-local:
	cargo run

#BACKTRACE_CMD ?= RUST_BACKTRACE=full
BACKTRACE_CMD ?= RUST_LOG=DEBUG
local-run:
	$(BACKTRACE_CMD) ./target/debug/$(BINARY_NAME)

lint:
	cargo fmt

#https://medium.com/@ericdreichert/how-to-print-during-rust-tests-619bdc7ccebc
local-test:
	cargo test -- --nocapture

run:
	docker run --rm -p 3000:3000 $(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION)

clean:
	cargo clean

publish: build
	docker push $(USERNAME)/$(APP_NAME):$(IMAGE_NAME)-$(VERSION)
