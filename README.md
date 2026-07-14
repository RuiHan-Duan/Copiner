# Copiner

Copiner 是面向企业内部非代码工作的本地优先 AI Agent 桌面工具。当前仓库落地了 M0/M1 的可扩展骨架：Harness、闭环工作流状态机、桌面 UI 和云端工作区边界。

## 技术栈

- `Desktop/`：Vue 3 + TypeScript + Vite，作为 Tauri WebView 界面
- `Crates/CoreEngine/`：Rust Agent 内核，无框架依赖的领域层
- `Crates/DesktopHost/`：Tauri 桌面宿主
- `WorkspaceService/`：云上 Workspace 边界（首版仅占位，不接企业数据）
- `Skills/`、`Hooks/`、`Memory/`：可移植 Harness 配置

## 快速开始

需要 Node.js 24、pnpm 11.12 和 Rust stable；Linux 构建 Tauri 还需安装其官方系统依赖。

```bash
make install
make build
make test
make lint
make dev
```

仅开发前端可运行 `cd Desktop && pnpm dev`。完整桌面调试运行 `make dev`。

目录职责、阶段范围和安全边界见 [Docs/00-项目介绍.md](Docs/00-项目介绍.md)。
