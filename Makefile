WASM_TARGET := wasm32v1-none
WASM_DIR    := target/$(WASM_TARGET)/release
CONTRACTS   := escrow stream vesting recurring batch_payout

.PHONY: all build test fmt fmt-check lint audit optimize specs clean ci

all: build test

# The integration-test crate is host-only (it needs std), so it is excluded
# from the wasm build rather than failing it.
build:
	cargo build --workspace --exclude sororail-integration-tests \
		--target $(WASM_TARGET) --release

# Tests run on the host, not on wasm -- soroban_sdk::testutils needs std.
test:
	cargo test --workspace

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets -- -D warnings

audit:
	cargo audit

# Strips and shrinks each contract wasm. Requires the `stellar` CLI.
optimize: build
	@for c in $(CONTRACTS); do \
		echo "optimizing $$c"; \
		stellar contract optimize --wasm $(WASM_DIR)/sororail_$$c.wasm || exit 1; \
	done

# Contract specs as JSON, attached to the GitHub Release.
specs: build
	@mkdir -p dist/specs
	@for c in $(CONTRACTS); do \
		stellar contract info interface --output json \
			--wasm $(WASM_DIR)/sororail_$$c.wasm > dist/specs/$$c.json || exit 1; \
	done
	@echo "specs written to dist/specs/"

ci: fmt-check lint test build

clean:
	cargo clean
	rm -rf dist
