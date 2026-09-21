.PHONY: build test clean clippy fmt check

build:
	cargo build --target wasm32v1-none --release

test:
	cargo test

clean:
	cargo clean

clippy:
	cargo clippy --all-targets -- -D warnings

fmt:
	cargo fmt --all

check:
	cargo check --all-targets
