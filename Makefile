run:
	cargo run

fmt:
	cargo fmt

test: fmt
	cargo test

dev: fmt
	cargo run

dc-build:
	docker-compose build

dc-up:
	docker-compose up -d