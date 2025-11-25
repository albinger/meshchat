all: clippy updeps debug

checks: format clippy

format:
	cargo fmt --all -- --check

clippy:
	cargo clippy --tests --no-deps --all-features --all-targets

updeps:
	cargo +nightly udeps

debug:
	cargo build

release:
	cargo build --release

run:
	cargo run --release