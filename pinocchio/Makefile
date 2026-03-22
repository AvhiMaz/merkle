.PHONY: build test format clean all

build:
	cargo build-sbf

test:
	cargo test

cu:
	cargo test -- --nocapture 2>&1 | grep -E "initialize CU|insert CU|verify CU"

format:
	cargo fmt --all

clean:
	cargo clean

all:
	make format && make build && make test && make cu
