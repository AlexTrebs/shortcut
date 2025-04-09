setup:
	pnpm install
	cargo binstall cargo-watch
	cargo binstall sqlx-cli
	sqlx db create
	sqlx migrate run
	cargo build

tailwind: 
	npx @tailwindcss/cli -i ./ui/styles/tailwind.css -o ./ui/assets/main.css

tailwind-watch:
	npx @tailwindcss/cli -i ./ui/styles/tailwind.css -o ./ui/assets/main.css --watch

server-watch:
	RUST_LOG=info cargo watch -x run
