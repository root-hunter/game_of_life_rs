build:
	wasm-pack build --release -d ./docs/pkg --target web
	
	rm ./docs/pkg/.gitignore

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