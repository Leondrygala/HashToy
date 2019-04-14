
.PHONY: all
all: test run

.PHONY: test
test:
	cargo test

.PHONY: run
run: 
	cargo run
