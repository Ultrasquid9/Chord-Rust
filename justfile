default: lint test

run:
	cargo run

lint:
	cargo fmt
	cargo clippy

test:
	cargo test
