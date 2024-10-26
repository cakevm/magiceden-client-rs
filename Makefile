.PHONY: build run test clean fmt clippy taplo deny-check

build:
	cargo build --all

release:
	cargo build --release

run:
	cargo run

test:
	cargo test

clean:
	cargo clean

fmt:
	cargo fmt

clippy:
	cargo clippy --all --all-features -- -D warnings

taplo:
	taplo format

deny-check:
	cargo deny --all-features check