test:
	cargo test
python:
	maturin develop --release --features python
wasm:
	wasm-pack build --target web --features wasm