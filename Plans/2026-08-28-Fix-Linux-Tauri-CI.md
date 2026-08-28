# 修复 Linux Tauri CI Plan

> 状态：等待推送确认
> 日期：2026-08-28
> 关联 PR：`#1 codex/local-harness-ui-redesign → main`

## 1. 问题与目标

PR #1 的 `Desktop` job 已通过，`Rust workspace` job 在 Ubuntu 24.04 执行 `cargo clippy --workspace --all-targets -- -D warnings` 时失败。日志显示 `glib-sys` 通过 `pkg-config` 找不到 `glib-2.0.pc`；这是 Tauri 2 在 Linux 编译所需的系统开发库缺失，不是 Rust 源码或 Clippy warning。

目标是在 GitHub Actions 的 Rust job 中显式安装 Tauri 2 官方 Debian/Ubuntu 前置依赖，使全 workspace 的 fmt、clippy、test 和 build 能在干净 Linux runner 上执行。

## 2. Todos

### T1 — 补齐 Ubuntu 系统依赖

修改 `.github/workflows/ci.yml`，在 Rust toolchain 后、Cargo 检查前增加一个命名步骤：

```text
sudo apt-get update
sudo apt-get install --no-install-recommends -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

依赖清单采用 Tauri 2 官方 Linux prerequisites，不只安装 `libglib2.0-dev`，避免随后继续缺失 WebKitGTK、AppIndicator、RSVG 或 OpenSSL 开发库。使用 `--no-install-recommends` 控制 runner 安装面，不修改 Cargo 依赖或锁文件。

### T2 — 同步构建文档

- `README.md`：将 Linux/Tauri 系统依赖从泛称改为可复制的 Ubuntu 安装命令；
- `Crates/DesktopHost/README.md`：说明 Linux 本地开发与 CI 需要 Tauri 原生依赖；
- `Docs/03-技术选型.md`：记录 CI 的 Ubuntu Tauri 前置依赖决策和日期；
- 本 Plan：追加真实验证和 Actions 结果。

不修改 `Docs/02-任务清单.md`，因为这是 M0-03 已有 CI 门禁的维护，不代表新里程碑能力完成。

### T3 — 本地验证

执行：

```text
corepack pnpm@11.12.0 --dir Desktop install --frozen-lockfile
corepack pnpm@11.12.0 --dir Desktop run typecheck
corepack pnpm@11.12.0 --dir Desktop run build
corepack pnpm@11.12.0 --dir Desktop run lint
corepack pnpm@11.12.0 --dir Desktop run check:consistency
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
git diff --check（逐路径复核，规避本机全量 diff 挂起）
```

本机为 macOS，不能证明 Ubuntu apt 安装链成功；Linux 结果只认 GitHub Actions 的新 run。

### T4 — 更新 PR 并观察 CI

本地验证通过后：

1. 只暂存本 Plan 列出的 workflow 与文档文件；
2. 创建独立修复提交；
3. 在推送前再次展示远端、分支、提交和影响，取得动作级确认；
4. 推送到 `origin/codex/local-harness-ui-redesign`，等待 PR #1 的 `Desktop` 与 `Rust workspace` 终态；
5. 若仍失败，只按新日志继续诊断，不扩大依赖清单或绕过检查。

## 3. 非目标

- 不跳过 DesktopHost、不从 workspace 排除 Tauri crate；
- 不把 `cargo clippy/test/build` 改为只检查 CoreEngine；
- 不降低 `-D warnings`、不关闭 CI job；
- 不修改产品代码、Cargo 依赖、锁文件或运行时契约；
- 不合并 PR、不发布安装包。

## 4. 风险与回滚

- 风险：apt 安装会增加 Rust job 的启动时间；用官方最小开发依赖和 `--no-install-recommends` 控制范围。
- 风险：`ubuntu-latest` 将来迁移镜像；依赖名称与 Tauri 2 WebKitGTK 4.1 基线保持显式，失败时可从日志定位。
- 回滚：删除 workflow 的安装步骤并回退对应文档即可，无数据迁移或产品行为影响。

## 5. 验收

- PR #1 的 `Desktop` 与 `Rust workspace` checks 均为 success；
- Rust job 顺序执行 fmt、clippy、test、build，不能只通过第一步；
- 本地验证通过，工作区无未说明改动；
- 文档命令与 workflow 实际依赖一致。

## 6. 确认

用户已确认进入 Execute。Plan 确认只授权本地 workflow、文档修改与验证；commit/push 仍在动作发生前单独确认。

## 7. 本地执行结果（2026-08-28）

已完成：

- Rust CI job 在 Cargo 检查前安装 Tauri 2 官方 Ubuntu 前置依赖；
- 根 README、DesktopHost README 与技术选型已同步；
- CI YAML 可由本机 YAML parser 读取；
- 逐路径 `git diff --check` 通过。

验证结果：

```text
corepack pnpm@11.12.0 --dir Desktop install --frozen-lockfile  PASS
corepack pnpm@11.12.0 --dir Desktop run typecheck              PASS
corepack pnpm@11.12.0 --dir Desktop run build                  PASS
corepack pnpm@11.12.0 --dir Desktop run lint                   PASS
corepack pnpm@11.12.0 --dir Desktop run check:consistency      PASS
cargo fmt --all -- --check                                     PASS
cargo clippy --workspace --all-targets -- -D warnings          PASS
cargo test --workspace                                         PASS (16 tests)
cargo build --workspace                                        PASS
```

待完成：创建独立修复提交；取得动作级确认后推送到 `origin/codex/local-harness-ui-redesign`，并以 PR #1 的新 GitHub Actions run 验证 Ubuntu 依赖链。
