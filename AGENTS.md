# AGENTS.md — Copiner Agent 入口

Copiner 是面向企业非代码工作的本地优先 AI Agent。当前是 M0/M1 可构建骨架，不得编造业务数据或声称已接入企业系统。

1. 所有任务先读 `Docs/00-项目介绍.md` 与 `Docs/01-开发规范.md`。
2. 范围和里程碑以 `Docs/02-任务清单.md` 为准；技术决策查 `Docs/03-技术选型.md`。
3. Agent 行为先读 `COWORK.md`，场景细则按其中路由渐进加载。
4. 外部副作用必须停在 Plan/Approval 边界，未经人工确认不得执行。
5. 敏感内容默认只走本地/内网 Provider；不确定时按敏感处理。
6. 完成必须有 build、test、lint 或可复现验收证据。

