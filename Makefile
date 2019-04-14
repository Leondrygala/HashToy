
.PHONY: all
all: test run

.PHONY: test
test:
	docker run -v $(PWD):/work rust:1.34 \
		cargo test --manifest-path /work/Cargo.toml

.PHONY: run
run: 
	docker run -v $(PWD):/work rust:1.34 \
		cargo run --manifest-path /work/Cargo.toml
