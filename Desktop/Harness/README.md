# Harness UI

## 职责

展示当前会话、任务归档、Skills 和 Memory 的入口。文件内容仍以仓库中的 `COWORK.md`、`Skills/`、`Hooks/`、`Memory/` 为事实来源。

## 当前实现

`src/Harness/HarnessNavigation.vue` 只有只读导航，没有加载、编辑、同步或索引能力。

## 安全边界

- Skills 与 Memory 的展示不代表已授权读取企业系统；
- 敏感 Memory 不得自动同步到外部服务；
- SessionStart Hook 必须先经过来源授权、脱敏和 Provider 路由判断。
