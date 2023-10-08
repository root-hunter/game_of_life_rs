build:
	wasm-pack build --release --target web
	mkdir -p ./static
	cp -r ./pkg ./static/
	cp ./html/* ./static/
	rm ./static/pkg/.gitignore

serve:
	python3 -m http.server
