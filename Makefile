.PHONY: build format

build:
	cargo build

format:
	rustfmt -v --edition 2021 src/main.rs
