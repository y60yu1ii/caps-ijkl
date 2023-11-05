all: build

build:
	cargo build --release

install: build
	sudo install target/release/caps-hjkl /usr/local/bin
	sudo install systemd/caps-hjkl.service /etc/systemd/system
	sudo systemctl enable caps-hjkl
	sudo systemctl start caps-hjkl

uninstall:
	sudo rm -f /usr/local/bin/caps-hjkl /etc/systemd/system/caps-hjkl.service
	sudo systemctl stop caps-hjkl
	sudo systemctl disable caps-hjkl
