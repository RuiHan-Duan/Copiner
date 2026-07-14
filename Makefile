.DEFAULT_GOAL := help
.PHONY: help install build test lint dev

help:
	@echo "Copiner: make install | build | test | lint | dev"
install:
	cd Desktop && pnpm install
build:
	cd Desktop && pnpm run build
	cargo build --workspace
test:
	cargo test --workspace
lint:
	cd Desktop && pnpm run typecheck
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets -- -D warnings
dev:
	cd Desktop && pnpm exec tauri dev --config ../Crates/DesktopHost/tauri.conf.json
