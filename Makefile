run:
	cargo run

fmt:
	cargo fmt

test: fmt
	cargo test

dev: fmt
	sqlx database create
	sqlx migrate run
	cargo run

dc-build:
	docker-compose build --no-cache

dc-up:
	docker-compose up -d

dc-down:
	docker-compose down