BINARY_NAME := taxes-redux

DRY_RUN ?= false
build:
	@if [ $(DRY_RUN) = true ]; then \
		echo dry run...; \
		cargo check; \
	else \
		cargo build; \
	fi

run:
	./target/debug/$(BINARY_NAME)

lint:
	cargo fmt

test:
	cargo test
