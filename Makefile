clean:
	rm -fr target

lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets

fmt:
	cargo fmt

test-unit:
	cargo test -- --nocapture

qa:\
test-unit \
lint

build: clean
	rustup target add wasm32-unknown-unknown
	env 'RUSTFLAGS=-C link-arg=-s' cargo build --verbose --target wasm32-unknown-unknown --release
