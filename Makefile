test:
	cargo test
python:
	maturin develop --release --features python
wasm:
	wasm-pack build --target web --features wasm
SITE_DIR ?= ../nicelgueta.github.io/rusty-random-slug
site: wasm
	mkdir -p $(SITE_DIR)/pkg
	cp index.html $(SITE_DIR)/
	cp pkg/rustyrs.js pkg/rustyrs_bg.wasm $(SITE_DIR)/pkg/
