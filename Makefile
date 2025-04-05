.PHONY: format setup tailwind-watch server-watch dev

format:
	pnpm run format

setup:
	pnpm install
	cargo binstall cargo-watch
	cargo build

tailwind: 
	npx @tailwindcss/cli -i ./ui/styles/tailwind.css -o ./ui/assets/main.css

tailwind-watch:
	npx @tailwindcss/cli -i ./ui/styles/tailwind.css -o ./ui/assets/main.css --watch

server-watch:
	RUST_LOG=info cargo watch -x run

dev:
	make server-watch & make tailwind-watch 