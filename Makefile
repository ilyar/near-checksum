clean:
	cargo clean
	rm -fr build

lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets

fmt:
	cargo fmt

test-contract:
	cargo test -- --nocapture

qa:\
test-contract \
lint

rustup:
	rustup component add clippy
	rustup component add rustfmt
	rustup component add rust-src
	rustup target add wasm32-unknown-unknown

check:
	cargo check

build/checksum.wasm:
	env 'RUSTFLAGS=-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	@mkdir build
	@mv target/wasm32-unknown-unknown/release/checksum.wasm build/checksum.wasm
	@du -b build/checksum.wasm
	@sha256sum build/checksum.wasm

deploy-force: build/checksum.wasm
	near dev-deploy --force --wasmFile build/checksum.wasm

build:\
build/checksum.wasm
