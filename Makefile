# Copiner unified build entry.
# Native commands remain available; these targets only coordinate Desktop and Rust.

.DEFAULT_GOAL := help
.PHONY: help install build test lint check clean dev workspace-run \
        desktop-install desktop-build desktop-test desktop-lint desktop-check \
        rust-build rust-test rust-lint

ifeq ($(OS),Windows_NT)
  RMRF = if exist $(1) rmdir /s /q $(1)
else
  RMRF = rm -rf $(1)
endif

help:
	@echo Copiner 构建入口
	@echo "  make install        安装 Desktop 冻结依赖"
	@echo "  make build          构建 Desktop 与 Rust workspace"
	@echo "  make test           执行 Desktop 骨架检查与 Rust 测试"
	@echo "  make lint           执行前端 lint、一致性检查、Rust fmt/clippy"
	@echo "  make check          build + test + lint"
	@echo "  make clean          清理生成物"
	@echo "  make dev            启动 Tauri 桌面调试"
	@echo "  make workspace-run  运行未配置的 WorkspaceService 骨架"

install: desktop-install
build: desktop-build rust-build
test: desktop-test rust-test
lint: desktop-lint rust-lint
check: build test lint

# ---- Desktop (Vue 3 + TypeScript + pnpm) ----
desktop-install:
	cd Desktop && pnpm install --frozen-lockfile
desktop-build:
	cd Desktop && pnpm run build
desktop-test: desktop-check
	@echo "Desktop: 当前无交互单测，以 typecheck、build、lint 和一致性检查验收"
desktop-lint:
	cd Desktop && pnpm run lint
	$(MAKE) desktop-check
desktop-check:
	cd Desktop && pnpm run typecheck
	cd Desktop && pnpm run check:consistency

# ---- Rust workspace (CoreEngine + DesktopHost + WorkspaceService) ----
rust-build:
	cargo build --workspace
rust-test:
	cargo test --workspace
rust-lint:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets -- -D warnings

dev:
	cd Crates/DesktopHost && ../../Desktop/node_modules/.bin/tauri dev
workspace-run:
	cargo run -p copiner-workspace-service

clean:
	cd Desktop && $(call RMRF,dist)
	cd Desktop && $(call RMRF,node_modules/.vite)
	cargo clean
