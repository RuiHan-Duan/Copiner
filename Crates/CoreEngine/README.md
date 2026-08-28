# CoreEngine

无 UI、无厂商 SDK 的 Copiner 领域内核。它定义工作流和安全决策，供 DesktopHost 或未来其他宿主复用。

## 模块

| 文件 | 职责 | 当前能力 |
|---|---|---|
| `workflow.rs` | Clarify → Todos → Plan → Approval → Execute → Feynman → Archive | 强制状态迁移 |
| `workflow_session.rs` | 本地单任务会话、可观察 snapshot 与事件记录 | 内存闭环，可供宿主驱动 |
| `policy.rs` | 数据敏感级别与 Provider 边界 | Unknown 默认禁止外部路由 |
| `provider.rs` | Provider 描述、注册与选择 | 内存注册表，无真实 SDK |
| `tool.rs` | Tool 权限、副作用和动作批准 | 内存注册表，无执行器 |
| `harness.rs` | Skill、Hook、Memory 元数据 | 内存目录，无文件加载 |

## 公共边界

现有 `Workflow`、`WorkflowState`、`WorkflowEvent`、`WorkflowError` 继续从 crate 根导出。`WorkflowSession` 是首个宿主可观察切片：保存任务标题、精确状态、唯一下一动作和内存事件记录，但不包含模型、工具或持久化。

## 安全不变量

- `Unknown`、`Internal`、`Sensitive` 不得路由到 External Provider；
- 任何本地或外部写操作都必须获得动作级 `Approved`；
- Provider 和 Tool 名称不可重复；
- 本 crate 不读取密钥、不联网、不访问文件或企业数据。
- `WorkflowSession` 完成归档后不可继续推进，必须由宿主重置并创建新任务。

## 验证

从仓库根执行 Rust workspace 的 fmt、clippy、test 和 build。
