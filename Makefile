build:
	cargo build
pong:
	cd examples && glib-compile-resources pong.xml
	cargo run --example pong
atlas:
	cd examples && glib-compile-resources atlas.xml
	cargo run --example atlas
lint:
	cargo fmt
	cargo clippy
