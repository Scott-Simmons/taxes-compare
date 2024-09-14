BINARY_NAME := taxes-redux

DRY_RUN ?= false
build:
	cargo update;
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

#https://medium.com/@ericdreichert/how-to-print-during-rust-tests-619bdc7ccebc
test:
	cargo test -- --nocapture
