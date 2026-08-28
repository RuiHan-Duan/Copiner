# Hooks

Hooks 描述会话启动时可注入的动态上下文来源。当前只有 schema 和脱敏示例，没有 Hook 执行器，也不会自动连接邮箱、Git 或企业系统。

## 文件

- `session-start.schema.json`：示例配置的结构约束；
- `session-start.example.json`：不含真实账号和数据的禁用示例。

## 安全流程

1. 来源先获得账号/系统范围授权；
2. 采集结果先做敏感分类和最小化；
3. Unknown 按 Sensitive 处理，只允许本地或内网 Provider；
4. 注入内容记录来源与采集时间，不把失败写成空结果；
5. Hook 只提供上下文，不绕过 Workflow 或 Tool 动作批准。

真实配置可能包含环境差异，不应提交账号、Token、Cookie、内网地址或生产内容。
