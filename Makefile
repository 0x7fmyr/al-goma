install:
	@which cargo > /dev/null || (echo "cargo not found, install Rust from https://rustup.rs" && exit 1)
	cargo build --release
	sudo cp target/release/al-goma /usr/local/bin/al-goma

uninstall:
	sudo rm -f /usr/local/bin/al-goma
	rm -rf ~/.config/al-goma
	rm -rf ~/.local/share/al-goma

