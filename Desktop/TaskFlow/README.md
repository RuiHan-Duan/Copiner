# TaskFlow

## 职责

展示闭环工作流阶段、Todo、Plan、审批等待和执行记录。状态迁移规则只存在于 `CoreEngine::workflow`，本模块不自行推进状态。

## 当前实现

- `src/Layout/WorkspaceHeader.vue`：以当前任务上下文和轻量阶段轨道呈现宿主返回的精确状态；
- `src/TaskFlow/TaskInspector.vue`：以 Run Control 工作面展示本地演示 Todo、精确状态、唯一下一动作和批准按钮；
- 阶段推进通过 DesktopHost 调用 `WorkflowSession::advance`，UI 不自行计算合法迁移；
- 本次批准只绑定“推进内存状态”，不授权模型、Tool 或外部副作用。

## 后续接入

后续仍需补充事件订阅、真实执行错误流和完整动作参数/有效期展示；当前单会话使用命令返回的 snapshot 刷新页面。
