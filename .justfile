default:
    just --list

install-tools:
    cargo install sqlx-cli

reset-db:
	docker compose down -v
	docker compose up -d
	sleep 2
	sqlx db create
	sqlx migrate run

dev:
	cargo watch -x run

sqlx-prepare:
	cargo sqlx prepare --merged