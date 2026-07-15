# Desktop

Copiner 的 Vue 3 + TypeScript WebView 展示层，由 Tauri `DesktopHost` 承载。当前提供可构建的本地优先空态壳，不调用模型或企业系统。

## 模块

| 模块 | 代码 | 职责 | 当前状态 |
|---|---|---|---|
| Layout | `src/Layout/` | 三栏桌面壳、品牌区、阶段头部 | 可用骨架 |
| Conversation | `src/Conversation/` | 对话空态、建议项、输入器 | 未配置 Provider 时禁用 |
| TaskFlow | `src/TaskFlow/` | Todo、Plan 与审批护栏展示 | 只读空态 |
| Harness | `src/Harness/` | 会话、归档、Skills、Memory 导航 | 只读导航 |
| Shared | `src/Shared/` | 展示契约、主题令牌 | 不含领域判断 |

Workflow、敏感路由和工具审批的事实来源是 `Crates/CoreEngine`。Desktop 中的 `ShellViewModel` 只是尚未接入 Tauri 命令前的空态展示模型。

## 命令

```text
pnpm install --frozen-lockfile
pnpm run dev
pnpm run typecheck
pnpm run build
pnpm run lint
pnpm run check:consistency
```

开发服务器固定 `5173`，构建产物位于 `Desktop/dist/` 且不入库。完整桌面调试从仓库根执行 `make dev`。

## 已知限制

- 没有 Provider 配置页、真实对话、状态持久化或 Tauri 事件绑定；
- 建议按钮、设置和输入器有意禁用，避免占位 UI 被误认为可执行能力；
- 未接入桌宠资产，当前菱形符号仅表达空态。
