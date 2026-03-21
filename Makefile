.PHONY: build test format

build:
	quasar build

test:
	quasar test

cu:
	cargo test -- --no-capture

format:
	cargo +nightly fmt --all

all:
	make format && make build && make test && make cu
