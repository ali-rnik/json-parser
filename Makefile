all:
	cargo build --release
	sudo install target/release/json-parser /usr/local/bin
