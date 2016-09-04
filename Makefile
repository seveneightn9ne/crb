.PHONY: all

all:
	cargo fmt
	cargo build

clean:
	cargo clean
