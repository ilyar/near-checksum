clean:
	cargo clean
	rm -fr build

lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets

fmt:
	cargo fmt

audit-fix:
	cargo audit fix

audit:
	cargo audit

test-contract: build/checksum.wasm
	cargo test --all

test-contract-unit:
	cargo test --lib

test:\
test-contract-unit

qa:\
lint \
test

fix:\
audit-fix\
fmt

rustup:
	rustup component add clippy
	rustup component add rustfmt
	rustup component add rust-src
	rustup target add wasm32-unknown-unknown
	cargo install cargo-audit --features=fix

check:
	cargo check

build/checksum.wasm:
	cargo build --target wasm32-unknown-unknown --release
	@mkdir build
	@mv target/wasm32-unknown-unknown/release/checksum.wasm build/checksum.wasm
	@du -b build/checksum.wasm
	@sha256sum build/checksum.wasm

deploy-force: build/checksum.wasm
	near dev-deploy --force --wasmFile build/checksum.wasm

build:\
build/checksum.wasm
