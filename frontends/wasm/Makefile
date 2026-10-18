install:
	cargo install cargo-server
	cargo install wasm-bindgen-cli
	cargo install cargo-watch
	# Binaryen is also needed
	# https://github.com/WebAssembly/binaryen

run: build opt
	cargo server --open

opt:
	du -h ./wasm/marioterm_bg.wasm
	wasm-opt -O ./wasm/marioterm_bg.wasm -o ./wasm/marioterm_bg.wasm
	du -h ./wasm/marioterm_bg.wasm

watch:
	cargo watch -- make build

build:
	wasm-bindgen ../target/wasm32-unknown-unknown/debug/marioterm.wasm --out-dir wasm --target web --no-typescript

build-release:
	wasm-bindgen ../target/wasm32-unknown-unknown/release/marioterm.wasm --out-dir wasm --target web --no-typescript