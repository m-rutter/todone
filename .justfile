default:
    just --list

install-tools:
    cargo install sqlx-cli cargo-watch

reset-db:
	docker compose down -v
	docker compose up -d
	sleep 2
	cd todone-backend && sqlx db create
	cd todone-backend && sqlx migrate run

dev:
	cd todone-backend && cargo watch -x run

sqlx-prepare:
	cargo sqlx prepare --merged