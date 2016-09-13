.PHONY: all

all:
	cargo fmt
	cargo build

clean:
	cargo clean

meep:
	clear
	make
	cargo run src/main.rs
