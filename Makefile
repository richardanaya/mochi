build:
	cargo build
pong:
	cd examples && glib-compile-resources pong.xml
	cargo run --example pong
lint:
	cargo fmt
	cargo clippy