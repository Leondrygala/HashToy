
.PHONY: all
all: test run

.PHONY: test
test:
	docker run -v $(PWD):/work rust:1.47 \
		cargo test --manifest-path /work/Cargo.toml

.PHONY: bench
bench:
	docker run -v $(PWD):/work rust:1.47 \
		cargo bench --manifest-path /work/Cargo.toml

.PHONY: run
run: 
	docker run -v $(PWD):/work rust:1.47 \
		cargo run --manifest-path /work/Cargo.toml
