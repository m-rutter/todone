default:
    just --list

install-tools:
    cargo install sqlx-cli

reset-db:
	docker compose down -v
	docker compose up -d
	sleep 2
	cd todone-backend && sqlx db create
	cd todone-backend && sqlx migrate run

dev:
	cargo watch -x run

sqlx-prepare:
	cargo sqlx prepare --merged