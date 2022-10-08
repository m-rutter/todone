default:
    just --list

install-tools:
    cargo install sqlx-cli

reset-db:
	docker compose down -v
	docker compose up -d
	sqlx db create
	sqlx migrate run

dev:
	cargo watch -x run