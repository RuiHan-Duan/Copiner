# Copiner 协作指南

Copiner 采用运行时边界自治、公共契约走 Plan、自动验证兜底的协作方式。当前仓库未配置模块负责人，不根据账号或提交记录猜测所有权。

## 日常流程

1. 从 `Docs/02-任务清单.md` 或 Issue 确认目标与里程碑编号。
2. 读取根和目标目录的 `AGENTS.md`、目标模块 README。
3. 单模块明确维护可直接列 Todos 后实施；跨界改动先在 `Plans/` 写 Plan 并等待确认。
4. 使用短分支，例如 `feat/m1-provider-policy`、`fix/desktop-build`、`docs/harness-guide`。
5. 执行目标边界的 build、test、lint；同步模块 README 和需求进度。
6. 外部副作用在动作发生前单独请求确认，不把 Plan 批准当成永久授权。

## 跨界判定

以下任一情况属于跨界：

- 同时修改 Desktop、CoreEngine、DesktopHost、WorkspaceService 中两个以上边界；
- 改变 Provider、Tool、Hook、Workflow 公共类型或序列化契约；
- 修改敏感路由、审批规则、鉴权、密钥或审计策略；
- 修改根 Cargo workspace、前端依赖、锁文件、Makefile、`build.ps1` 或 CI；
- 修改 `Docs/02-任务清单.md`、`Docs/03-技术选型.md` 或目录结构。

跨界 PR 重点确认调用方、提供方、安全影响、验证证据和回滚方式，不要求逐行复核所有生成代码。

## 提交约定

提交保持小而单一，推荐 Conventional Commits：

```text
feat(core): add provider routing policy
fix(desktop): preserve empty state on narrow screens
docs(harness): clarify session hook boundary
build(ci): enforce frozen pnpm lockfile
```

禁止强推共享 `main`，不提交生成物、密钥和无法构建的中间状态。依赖或工具链变化必须与锁文件、文档、构建入口和 CI 同步提交。

## 完成定义

- 目标行为可复现，安全边界没有退化；
- 相关 build/test/lint 或健康检查真实通过；
- 无生产敏感数据、密钥、生成物和伪造业务结果；
- 模块 README 与代码实景一致；完整能力才更新任务清单为完成；
- 跨界变更说明影响面和回滚方式。
