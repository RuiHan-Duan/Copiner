# Desktop Agent 指令

继承根 `AGENTS.md`。修改界面前先读 `Desktop/README.md` 和目标模块 README。

## 边界

- Desktop 是 Vue 3 WebView 展示层，只消费 CoreEngine/DesktopHost 暴露的契约，不复制工作流、敏感路由或审批规则。
- 无 Provider、任务或企业数据时必须显示空态，不生成模拟指标。
- `src/Shared/` 只放展示契约、主题与无副作用工具；业务模块不得循环依赖。
- Tauri 调用集中在适配层。组件不直接读取密钥、文件系统或企业系统。

## 目录路由

| 范围 | 先读 |
|---|---|
| `src/Conversation/` | `Desktop/Conversation/README.md` |
| `src/TaskFlow/` | `Desktop/TaskFlow/README.md` |
| `src/Harness/` | `Desktop/Harness/README.md` |
| `src/Layout/`、`src/Shared/` | `Desktop/README.md` |

## 验证

```text
pnpm install --frozen-lockfile
pnpm run typecheck
pnpm run build
pnpm run lint
pnpm run check:consistency
```

布局或交互变化还需在浏览器或 Tauri WebView 中实测宽屏与窄屏空态。
