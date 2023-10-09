build:
	wasm-pack build --release --target web
	mkdir -p ./static
	cp -r ./pkg ./static/
	cp ./html/script.js ./static/
	cp ./html/style.css ./static/
	cp ./html/manifest.json ./static/

	cp ./html/index.html ./index.html
	rm ./static/pkg/.gitignore

serve:
	#python3 -m http.server
	cd server; npm run start

install:
	#INSTALL RUST
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

	#INSTALL NODE AND NPM
	sudo apt install nodejs
	sudo apt install npm

	#INSTALL SERVER DEPS
	cd server; npm install