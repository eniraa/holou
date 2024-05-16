default: build_release

clean:
	@rm -rf target/*
	@cargo clean
fix:
	@python3 scripts/update_commands.py
lint: fix
	@cargo check
	@cargo fmt --all -- --check
	@cargo clippy --all-targets --all-features -- -D warnings
build_release: lint
	@echo "Building release: $(VERSION)"
	@cargo build --release
build_debug: lint
	@echo "Building debug"
	@cargo build
run: build_debug
	@echo "Running debug"
	@cargo run
