# Conversation

## 职责

承载用户任务输入、对话历史和未配置时的真实空态。未来只通过 DesktopHost 适配层提交已标注敏感级别的请求，不直接持有 Provider Key。

## 当前实现

- `src/Conversation/EmptyConversation.vue`：以 Copiner 字标、本地能力说明和起始原则构成空态；运行后展示任务标题、事件轨道和真实完成摘要；
- `src/Conversation/MessageComposer.vue`：作为主要操作区，仅在 Tauri 本地宿主可用且无活动会话时创建演示任务；
- 建议项只作视觉占位，不触发 Skill 或企业系统。

当前输入不是模型对话：它只创建进程内 `WorkflowSession`。浏览器开发模式保持只读预览，避免用前端模拟器冒充宿主执行。

## 扩展边界

接入对话前必须先完成 Provider 路由、敏感标签和会话契约评审。发送邮件、提单等动作不能从输入组件直接执行，必须进入 CoreEngine 审批流程。
