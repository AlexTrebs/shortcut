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

build-windows: 
	echo "🦀 Building Rust project..."
	rustup target add x86_64-pc-windows-gnu  
	cargo build --release --target x86_64-pc-windows-gnu

	# Linux commands (without inno):
	VERSION=$(grep '^version' Cargo.toml | head -n1 | cut -d '"' -f2)
	sed "s/{#AppVersion}/$VERSION/" build/windows_template.iss > build/windows_installer.iss

	# Window commands:
	# $versionLine = Get-Content Cargo.toml | Where-Object { $_ -match '^version\s*=' } | Select-Object -First 1
	# $version = $versionLine -replace '^version\s*=\s*"(.*)"', '$1'
	# (Get-Content build\windows_template.iss) -replace '\{#AppVersion\}', $version | Set-Content build\windows_installer.iss
	# & "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" ".\build\windows_installer.iss"

	echo "✅ Done!"

build-linux: 
	rustup target add x86_64-unknown-linux-gnu 
	cargo build --release --target x86_64-unknown-linux-gnu
