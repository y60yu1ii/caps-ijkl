all: build

build:
	cargo build --release

install: build
	sudo install target/release/caps-ijkl /usr/local/bin
	sudo install systemd/caps-ijkl.service /etc/systemd/system
	sudo systemctl enable caps-ijkl
	sudo systemctl start caps-ijkl

uninstall:
	sudo rm -f /usr/local/bin/caps-ijkl /etc/systemd/system/caps-ijkl.service
	sudo systemctl stop caps-ijkl
	sudo systemctl disable caps-ijkl
