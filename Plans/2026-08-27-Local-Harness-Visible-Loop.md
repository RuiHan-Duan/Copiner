# Copiner 本地 Harness 可见闭环 Plan

> 状态：已完成
> 日期：2026-08-27
> 对应里程碑：M1-06 DesktopHost 与 UI 状态绑定（首个可见切片）
> 参考实现：DeepSeek Harness、xAI Grok Build

## 1. 目标

在不接入真实模型、不执行文件或企业系统工具、不伪造业务结果的前提下，交付一个可从 Tauri 桌面端实际运行和观察的最小 Harness 闭环：

1. 用户输入一个本地演示任务；
2. Desktop 通过 Tauri 命令创建由 CoreEngine 驱动的 Workflow；
3. 用户逐步确认 Clarify → Todos → Plan → Approval → Execute → Feynman → Archive；
4. UI 实时展示当前阶段、Todo/Plan、事件时间线和明确的安全边界；
5. 完成后显示真实的运行摘要：本次只验证 Harness 状态流与宿主绑定，未调用模型、工具或外部系统。

该成果的价值是先证明 Copiner 的 UI、宿主与领域内核已经纵向贯通，为下一步真实 Provider、流式输出和 Tool 执行器建立稳定落点。

## 2. 参考实现结论

### DeepSeek Harness

- Agent 接口与具体 Agent Loop 分离，循环可以替换；
- Session 事件是可持久、可重放的事实，运行中的 Agent 事件只负责实时协调；
- 模型、工具、会话、UI 等能力通过明确扩展点组合，而不是都塞进循环；
- 一个 step 由一次模型请求和随后的工具调用组成，一个 turn 可以包含多个 step。

本次借鉴：CoreEngine 继续作为状态事实来源；DesktopHost 输出稳定 snapshot/event DTO；Desktop 只渲染，不复制迁移规则。

### Grok Build

- TUI、Agent Runtime、Tools、Workspace 分 crate，UI 与执行层分离；
- 交互式 TUI、headless 和 ACP 客户端共享同一 Agent Runtime；
- 工具执行、权限请求和会话更新通过协议显式暴露给客户端。

本次借鉴：先做“同一运行时、一个桌面客户端”的最小协议切片；不引入 ACP、leader 进程、MCP、沙箱或 Grok 的工具实现。

## 3. Todos 与改动边界

### T1 — CoreEngine：可观察 Workflow 会话

范围：`Crates/CoreEngine/`

- 在现有 `Workflow` 状态机之上增加最小本地运行会话，保存任务标题和已发生的领域事件；
- 提供只读 snapshot，包含当前精确状态、可执行的下一动作和事件记录；
- 所有迁移仍调用现有 `Workflow::apply`，不新增绕过 Approval 的路径；
- 增加合法闭环、非法动作、重置和 snapshot 的单元测试；
- 同步 `Crates/CoreEngine/README.md`。

不包含：模型消息、token、工具执行、网络、文件读写、持久化。

### T2 — DesktopHost：Tauri 命令适配

范围：`Crates/DesktopHost/`

- 以 Tauri managed state 保存单个本地演示会话；
- 暴露创建会话、读取 snapshot、推进下一动作、重置会话的命令；
- 将 CoreEngine 类型映射为可序列化 DTO，不在命令中重写领域规则；
- 错误返回结构化 code/message，不记录任务正文；
- 增加命令层 Rust 测试并同步 `Crates/DesktopHost/README.md`。

不包含：后台常驻 Agent、并发多会话、磁盘持久化、系统密钥链。

### T3 — Desktop：可运行的本地演示界面

范围：`Desktop/`

- 增加集中式 Tauri adapter；浏览器开发模式提供明确的“仅预览，需在 Tauri 中运行”状态，不模拟宿主结果；
- 输入器允许创建本地演示任务，并明确显示“未连接模型”；
- Conversation 展示任务与运行事件，TaskFlow 展示 Todo/Plan/批准边界；
- 只有 CoreEngine snapshot 声明允许时才显示相应动作按钮；
- 宽屏和窄屏均能看到当前阶段与运行结果，键盘和焦点状态可用；
- 同步 Desktop 及 Conversation/TaskFlow README。

不包含：AI 生成文案、假对话、硬编码业务指标、Markdown 渲染、设置页。

### T4 — 依赖、文档与里程碑状态

- 为 Desktop 增加官方 `@tauri-apps/api` 调用依赖并更新唯一 pnpm 锁文件；
- 为 DesktopHost 增加 DTO 序列化所需的最小 serde 依赖并更新 Cargo 锁文件；
- 修正 Tauri CLI 项目目录和 `beforeDevCommand` 的既有路径错误，确保 `make dev` 能从真实配置目录启动；
- 同步 `Docs/00-项目介绍.md`、`Docs/02-任务清单.md`、`Docs/03-技术选型.md` 和根 README；
- M1-06 仍保持未完成，只记录“本地单会话 snapshot/命令/UI 绑定已落地”，因为事件订阅、真实错误流和审批 UI 完整契约仍未完成。

## 4. 用户可见验收

从仓库根执行 `make dev` 后，可完成以下实际操作：

1. 看到“本地 Harness · 未连接模型”状态；
2. 输入任务标题并创建会话；
3. 依次看到 Todos、Plan、等待批准、Execute、Feynman、Archive；
4. 未批准时不能进入 Execute；
5. 每次推进都出现由宿主返回的事件记录；
6. 归档后看到“状态闭环已运行，模型/工具/外部系统均未调用”的真实摘要；
7. 重置后回到 Clarify 空态。

## 5. 验证

实施完成后至少执行：

```text
pnpm --dir Desktop install --frozen-lockfile
pnpm --dir Desktop run typecheck
pnpm --dir Desktop run build
pnpm --dir Desktop run lint
pnpm --dir Desktop run check:consistency
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
git diff --check
```

交互验证：

- 在 Tauri 中走完一次完整闭环；
- 尝试跳过 Approval，确认 CoreEngine 拒绝；
- 浏览器预览不得显示伪造的成功运行；
- 分别检查约 1280px 和 700px 布局、键盘焦点及 console 错误。

## 6. 风险与约束

- 这是 Harness 纵向切片，不是模型能力演示；界面必须持续显示“未连接模型”。
- 本地演示会话只存在内存中，应用重启后清空。
- 不复制 DeepSeek/Grok 源码，只借鉴其分层和事件边界；具体实现遵循 Copiner 现有 MIT 仓库结构。
- 若实施中需要改变现有 Workflow 状态语义、引入网络 SDK、加入 Tool 执行或持久化，必须先修订本 Plan 并重新确认。

## 7. 回滚

无数据迁移、无外部系统调用。可按 T1–T4 分组回滚；CoreEngine、DesktopHost 和 Desktop 的契约变更应成组撤回，避免 UI 与宿主版本不一致。

## 8. 待确认

用户已于 2026-08-27 确认进入 Execute。授权范围仅包含修改本地仓库并运行本地验证，不包含提交、push、PR、发布或任何外部数据操作。

## 9. 执行结果

### 已完成范围

- T1：新增 `WorkflowSession`、唯一下一动作、内存事件记录和完整闭环/非法跳转测试；批准仍由既有 `Workflow::apply` 强制。
- T2：DesktopHost 新增单会话 managed state，以及创建、snapshot、推进和重置 Tauri 命令；DTO 和错误均为结构化类型。
- T3：Desktop 新增集中式 Tauri adapter、本地任务输入、事件时间线、运行摘要、Todo/Plan、批准按钮和新任务重置；浏览器模式不模拟宿主。
- T4：加入 `@tauri-apps/api` 与 serde，更新锁文件和受影响文档；同时修复既有 Tauri CLI 项目目录错误，使 `make dev` 可以真实启动窗口。

### 验证证据

2026-08-27 实际执行：

```text
pnpm 11.12.0 frozen install：通过
vue-tsc --noEmit：通过
Vite production build：通过，29 modules transformed
ESLint --max-warnings 0：通过
工程一致性检查：通过
cargo fmt --check：通过
cargo clippy --workspace --all-targets -D warnings：通过
cargo test --workspace：16 passed / 0 failed
cargo build --workspace：通过
git diff --check：通过
```

交互与运行：

- 浏览器预览 1280×800：无横向溢出、console 0 warning / 0 error，并明确显示“浏览器预览 · 需 Tauri”；
- 浏览器预览 700×900：修复并复验空态裁切，页面宽度与 viewport 同为 700px，Todo/Plan 保持可见；
- `make dev`：Vite 启动于 `http://localhost:5173/`，Rust binary 编译完成并运行 `target/debug/copiner-desktop-host`；
- macOS 上实际出现 1224×780 的 `Copiner` 窗口，界面显示“本地 Harness · 未连接模型”，证明 Desktop 已通过宿主状态检查而非浏览器预览分支。

### 限制

- 当前 macOS 会话未授予终端辅助功能权限，因此无法用系统 UI 自动化逐个点击 Tauri WebView；完整状态闭环由 CoreEngine 13 项测试中的 3 项会话测试和 DesktopHost 2 项命令测试覆盖，桌面初始绑定与可见布局已实机验证。
- 会话仅存在内存，应用重启后清空；仍无模型、Tool 执行器、事件订阅或持久化。
- 本次没有 commit、push、PR、发布或外部数据操作。
