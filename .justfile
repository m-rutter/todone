default:
    just --list

install-tools:
	cargo install sqlx-cli cargo-watch 
	cargo install --locked trunk
	cargo install --locked wasm-bindgen-cli
	rustup target add wasm32-unknown-unknown

reset-db:
	docker compose down -v
	docker compose up -d
	sleep 2
	cd todone-backend && sqlx db create
	cd todone-backend && sqlx migrate run

dev:
	cd todone-backend && cargo watch -x run

dev-fe:
	cd todone-frontend && cargo watch -x run


sqlx-prepare:
	cd todone-backend && cargo sqlx prepare --merged