build:
	wasm-pack build --release --target web

serve:
	python3 -m http.server
