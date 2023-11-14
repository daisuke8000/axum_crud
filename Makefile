run:
	cargo run

fmt:
	cargo fmt

test: fmt
	cargo test

dev: fmt
	cargo run