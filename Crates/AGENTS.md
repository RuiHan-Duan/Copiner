# Rust Crates Agent 指令

继承根 `AGENTS.md`。修改 crate 前先读对应 README。

## 依赖方向

```text
DesktopHost ──> CoreEngine
WorkspaceService（独立 workspace member，不依赖 DesktopHost）
```

- `CoreEngine` 是无 UI、无厂商 SDK 的领域层，承载 Workflow、Provider/Tool 抽象和安全策略。
- `DesktopHost` 只负责 Tauri 生命周期、命令参数转换和本地适配，不重写领域规则。
- 新 Provider、数据库、网络或企业系统 SDK 属于技术选型变化，必须先走 Plan。
- 错误使用可比较的结构化类型；安全拒绝应可测试，不以日志字符串代替规则。

## 验证

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```
