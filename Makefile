SHELL := /bin/bash
TARGETS := throwaway-doi-check

.PHONY: all
all: $(TARGETS)

throwaway-doi-check: src/main.rs
	cargo build --release --bin throwaway-check-doi
	@cp target/release/throwaway-check-doi .

.PHONY: clean
clean:
	cargo clean
	rm -f $(TARGETS)

