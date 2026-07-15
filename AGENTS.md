# AGENTS.md — Copiner Agent 入口

Copiner 是面向企业非代码工作的本地优先 AI Agent。当前处于 M0/M1 工程骨架阶段，不得编造业务数据或声称已接入模型、RAG、邮箱、问题单或其他企业系统。

## 开始前按任务读取

1. 所有任务先读 `Docs/00-项目介绍.md` 与 `Docs/01-开发规范.md`。
2. 范围和里程碑只认 `Docs/02-任务清单.md`；技术决策查 `Docs/03-技术选型.md`。
3. 产品内 Agent 行为先读 `COWORK.md`，场景细则按其中路由渐进加载。
4. 进入代码边界后读取对应局部指令和模块 README。

| 改动范围 | 必读局部指令 |
|---|---|
| `Desktop/` | `Desktop/AGENTS.md` |
| `Crates/` | `Crates/AGENTS.md` |
| `WorkspaceService/` | `WorkspaceService/AGENTS.md` |
| `Skills/`、`Hooks/`、`Memory/` | `COWORK.md` 与目标文件 README/SKILL |

引用路径、模块或命令前先用 `rg` / `rg --files` 验证实景。文档与代码不一致时，以代码和真实执行结果为准，并在同次改动中同步文档。

## 改动边界

- **只读任务**：直接检查并报告证据，不自动实施修复。
- **单模块维护**：需求明确且不改变公共契约时可实施，必须执行目标模块验证并同步 README。
- **跨界改动**：目录、公共接口、安全策略、根依赖、构建、CI、需求基线或技术选型变化，必须先在 `Plans/` 写 Plan，经人工确认后执行。
- **外部副作用**：发送、提交、推送、合并、删除外部数据等动作，即使已有 Plan 也必须在动作发生前逐次确认。

## 不可妥协规则

1. 未确认的 Plan 不得进入 Execute；审批必须与具体动作和当前参数绑定。
2. 敏感级别不确定时按敏感处理，只能路由到本地或企业内网 Provider。
3. UI 无真实数据时展示空态；禁止随机数、硬编码指标或假接口冒充结果。
4. 领域规则放在 `CoreEngine`；Vue、Tauri 命令和 Workspace 只做展示或适配。
5. 密钥只从环境或系统密钥链读取，不提交 `.env`、Token、证书或生产样本。
6. 完成必须有 build、test、lint、健康检查或其他可复现证据，不能用 Agent 自述替代。

## 文档影响矩阵

| 改动 | 必须同步 |
|---|---|
| 模块代码 | 模块 README |
| 完整里程碑能力 | 模块 README + `Docs/02-任务清单.md` |
| 依赖、框架、包管理器 | 锁文件 + `Docs/03-技术选型.md` + README/AGENTS + CI |
| Provider/Tool/Hook 公共契约 | 调用方、提供方、模块 README 与安全说明 |
| 构建、启动、CI | 根 README + 局部 README/AGENTS + 一致性检查 |
| 目录结构 | `Docs/00-项目介绍.md` + AGENTS + 相关 README |
