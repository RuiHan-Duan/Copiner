# Conversation

## 职责

承载用户任务输入、对话历史和未配置时的真实空态。未来只通过 DesktopHost 适配层提交已标注敏感级别的请求，不直接持有 Provider Key。

## 当前实现

- `src/Conversation/EmptyConversation.vue`：说明 Clarify → Plan → Execute 的基本预期；
- `src/Conversation/MessageComposer.vue`：Provider 未配置时禁用输入和发送；
- 建议项只作视觉占位，不触发 Skill 或企业系统。

## 扩展边界

接入对话前必须先完成 Provider 路由、敏感标签和会话契约评审。发送邮件、提单等动作不能从输入组件直接执行，必须进入 CoreEngine 审批流程。
