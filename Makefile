.PHONY: build test format

build:
	quasar build

test:
	quasar test

format:
	cargo +nightly fmt --all

all:
	make format && make build && make test 
