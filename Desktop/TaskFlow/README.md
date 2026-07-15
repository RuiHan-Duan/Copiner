# TaskFlow

## 职责

展示闭环工作流阶段、Todo、Plan、审批等待和执行记录。状态迁移规则只存在于 `CoreEngine::workflow`，本模块不自行推进状态。

## 当前实现

- `src/Layout/WorkspaceHeader.vue`：四个产品级可见阶段；
- `src/TaskFlow/TaskInspector.vue`：Todo/Plan 空态和外部副作用护栏；
- 当前固定显示 Clarify，对应尚未接入宿主状态的真实情况。

## 后续接入

DesktopHost 提供只读 workflow snapshot 后，由页面订阅状态并渲染；审批控件必须显示具体动作、目标、参数和有效期，不能提供模糊的永久批准。
