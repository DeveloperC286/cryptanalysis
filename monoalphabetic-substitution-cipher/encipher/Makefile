# Use --locked in CI to ensure reproducible builds
CARGO_LOCKED := $(if $(CI),--locked,)

.PHONY: default
default: compile

.PHONY: check-rust-formatting
check-rust-formatting:
	cargo fmt --all -- --check --config=group_imports=StdExternalCrate

.PHONY: fix-rust-formatting
fix-rust-formatting:
	cargo fmt --all -- --config=group_imports=StdExternalCrate

.PHONY: check-rust-linting
check-rust-linting:
	cargo clippy --verbose $(CARGO_LOCKED) -- -D warnings

.PHONY: compile
compile:
	cargo build --verbose $(CARGO_LOCKED)

.PHONY: unit-test
unit-test:
	cargo test --verbose $(CARGO_LOCKED)
