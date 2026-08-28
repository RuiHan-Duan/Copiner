# Desktop

Copiner 的 Vue 3 + TypeScript WebView 展示层，由 Tauri `DesktopHost` 承载。当前提供可构建的本地优先空态壳，不调用模型或企业系统。

## 模块

| 模块 | 代码 | 职责 | 当前状态 |
|---|---|---|---|
| Layout | `src/Layout/` | 响应式桌面壳、品牌区、阶段头部 | 可用浅色工作面 |
| Conversation | `src/Conversation/` | 本地任务输入、事件记录、运行摘要 | 已绑定单会话 Harness；无模型对话 |
| TaskFlow | `src/TaskFlow/` | Todo、Plan 与审批护栏展示 | 可推进本地状态闭环 |
| Harness | `src/Harness/` | 会话、归档、Skills、Memory 导航 | 只读导航 |
| Shared | `src/Shared/` | 展示契约、主题令牌、Copiner 品牌标记 | 不含领域判断 |

Workflow、敏感路由和工具审批的事实来源是 `Crates/CoreEngine`。Desktop 通过 `src/Shared/local-harness-client.ts` 集中调用 Tauri 命令；浏览器模式只提供视觉预览，不模拟宿主响应。

## 视觉基线

- 品牌识别直接取自 `Copiner` 字名，以定制大写 `C` 和完整 wordmark 为核心，不推导名称词源或架构隐喻；
- UI 使用瓷白画布、白色工作面、深蓝灰文字、钴蓝强调和浅蓝阴影；
- 宽屏为工作区与运行控制并列布局，普通笔记本收窄导航，窄屏改为单列且保留所有推进动作；
- 品牌标记由 `src/Shared/BrandMark.vue` 统一渲染，主题与响应式规则集中在 `src/Shared/theme.css`。

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

- 没有 Provider 配置页、真实模型对话、状态持久化或 Tauri 事件订阅；
- 当前只支持一个进程内演示任务，应用重启后状态清空；
- 场景建议仍禁用，避免占位入口被误认为企业系统能力；
- 未接入桌宠资产；当前品牌标记只用于产品识别，不代表 Agent 人格或运行能力。
