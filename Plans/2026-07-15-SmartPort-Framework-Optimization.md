# Copiner 工程框架细化 Plan

> 状态：已完成
> 日期：2026-07-15
> 参考：SmartPort 可构建骨架、`Copiner-项目规划.md`、`Docs/02-任务清单.md`

## 1. 背景与目标

当前 Copiner 已有正确的运行时边界和最小可构建骨架，但详细度仍停留在“顶层目录 + 单文件占位”：

- `Desktop/src/App.vue` 同时承载布局、工作流导航、空态和安全提示，缺少模块边界；
- `CoreEngine/src/lib.rs` 同时定义状态、事件、错误和状态机，Provider、Tool、Harness、安全策略尚无可扩展接口；
- `DesktopHost`、`WorkspaceService` 仅有启动占位，缺少职责、依赖方向和扩展点说明；
- 根构建只提供粗粒度入口，前端没有锁文件、lint 和工程一致性检查；
- 缺少运行时局部 `AGENTS.md`、模块 README、AI 协作指南、贡献规范和 PR/Issue 模板；
- CI 使用非冻结依赖安装，无法保证本地、文档与流水线长期一致。

本次目标不是增加业务功能，而是把 Copiner 细化为与 SmartPort 同等级、同时符合 Copiner Harness 与安全规划的 M0/M1 工程框架：目录可导航、边界可约束、扩展点可编译、安全规则可测试、构建可复现、文档与实景可自动核对。

## 2. 设计原则

1. 顶层仍按运行时和可移植配置边界组织，不照搬 SmartPort 的 Java/Python 业务栈。
2. `CoreEngine` 保存领域规则；Vue、Tauri 和 Workspace 只做展示或适配。
3. Provider、Tool、Hook 只落接口、元数据、策略和测试，不接真实模型或企业系统。
4. 敏感内容默认拒绝外部 Provider；带外部副作用的 Tool 默认要求动作级批准。
5. 无数据继续展示空态，不加入演示数字、伪接口或假企业数据。
6. 文档描述“当前已实现”和“未来规划”时必须分开，避免占位骨架被误认为已接入。

## 3. 目标结构

```text
Copiner/
├── AGENTS.md                     全局 Agent 路由与跨界规则
├── COWORK.md                     产品内全局 Agent 铁律（保持精简）
├── CONTRIBUTING.md               分支、提交、跨界协作与完成定义
├── Makefile / build.ps1          macOS/Linux 与 Windows 统一入口
├── Scripts/
│   └── check-project-consistency.mjs
├── Desktop/
│   ├── AGENTS.md / README.md
│   ├── Conversation/README.md    对话与输入模块说明
│   ├── TaskFlow/README.md        Clarify→Archive 可视状态说明
│   ├── Harness/README.md         Skills/Memory/Settings 展示边界
│   └── src/
│       ├── App.vue
│       ├── Layout/               三栏桌面壳
│       ├── Conversation/         空态、建议项、输入器
│       ├── TaskFlow/             阶段条、Todo、Plan、安全状态
│       ├── Harness/              Harness 导航占位
│       └── Shared/               前端只读契约与主题令牌
├── Crates/
│   ├── AGENTS.md
│   ├── CoreEngine/
│   │   ├── README.md
│   │   └── src/
│   │       ├── workflow.rs       闭环状态机
│   │       ├── policy.rs         敏感路由与副作用审批规则
│   │       ├── provider.rs       Provider 能力与路由接口
│   │       ├── tool.rs           Tool 描述、权限与注册表
│   │       ├── harness.rs        Skill/Hook/Memory 元数据边界
│   │       └── lib.rs            稳定公共导出
│   └── DesktopHost/
│       ├── README.md
│       └── src/                  Tauri 启动与命令适配
├── WorkspaceService/
│   ├── AGENTS.md / README.md
│   └── src/                      能力声明与未配置空实现
├── Skills/                       四个场景手册，统一章节模板
├── Hooks/                        schema、脱敏示例和加载边界
├── Memory/                       可移植文本格式和示例
├── References/                   引用索引与敏感数据规则
├── Docs/
│   ├── 00-项目介绍.md            完整架构、目录、模块映射、现状
│   ├── 01-开发规范.md            任务分级、跨界流程、完成定义
│   ├── 02-任务清单.md            M0/M1 细分编号和真实进度
│   ├── 03-技术选型.md            版本、构建、依赖方向、暂缓项
│   └── 04-AI辅助开发指南.md      最小读取路径与验证矩阵
├── Plans/                        跨界 Plan 与真实验收记录
└── .github/                      CI、PR 模板、Issue 模板
```

目录名可能在实施时按 Rust/TypeScript 语言惯例做小幅调整，但不得改变上述职责和依赖方向；若职责变化，需要先修订本 Plan 并重新确认。

## 4. Todos

### T1 — 工程治理入口

- 细化根 `AGENTS.md`：加入任务分级、局部指令路由、不可妥协规则和文档影响矩阵。
- 新增 `Desktop/AGENTS.md`、`Crates/AGENTS.md`、`WorkspaceService/AGENTS.md`。
- 新增 `CONTRIBUTING.md`、PR 模板和需求/缺陷/架构变更 Issue 模板。
- 不创建带虚构负责人的 CODEOWNERS；负责人确认后另行补充。

### T2 — Desktop 模块化骨架

- 将单体 `App.vue` 拆为 Layout、Conversation、TaskFlow、Harness、Shared 模块。
- 保持现有四阶段空态体验和“Provider 未配置”真实状态，不增加模拟业务结果。
- 给三个产品模块补 README，明确职责、输入输出、当前状态、扩展位置和验证方式。
- 增加 ESLint、类型检查、构建和工程一致性脚本入口。

### T3 — CoreEngine 可扩展内核

- 拆分现有工作流状态机，保持所有公开行为兼容。
- 定义 Provider 能力、数据敏感级别和路由决策接口；敏感数据不得选择外部 Provider。
- 定义 Tool 权限、副作用级别、动作批准状态和注册表；未批准副作用不得进入执行。
- 定义 Harness 中 Skill、Hook、Memory 的元数据接口，不读取真实账号或企业内容。
- 为合法/非法状态迁移、敏感路由、Tool 重名和审批边界补 Rust 单元测试。

### T4 — 运行时适配边界

- 为 `DesktopHost` 补充 README 和最小命令适配结构，禁止把领域判断写进 Tauri 命令。
- 将 `WorkspaceService` 从单行占位细化为可测试的能力声明；RAG、队列、记忆索引和企业网关保持“未配置/不可用”。
- 明确 DesktopHost 只依赖 CoreEngine，WorkspaceService 不反向依赖桌面 UI。

### T5 — Harness 文件规范

- 为 Skills 统一“适用范围 / 输入 / 流程 / 输出 / 审批点 / 禁令 / References”章节。
- 为 Hook 增加可校验 schema 和脱敏示例，明确 SessionStart 只声明上下文来源，不自动连接外部系统。
- 细化 Memory 和 References 的文本格式、生命周期、删除/迁移和敏感规则。
- 保持 `COWORK.md` 精简，只承担全局行为与渐进加载路由。

### T6 — 可复现工具链

- 固定 Node 24、pnpm 11.12.0、Vite 6 和 Rust stable 的文档/配置一致性。
- 生成并提交唯一 `Desktop/pnpm-lock.yaml`，切换本地与 CI 为 frozen lockfile。
- 细化 Makefile 目标：全量入口、Desktop/Rust 分栈入口、开发启动、工程一致性和清理。
- 增加 Windows PowerShell 7 等价入口 `build.ps1`。
- CI 拆分 Desktop 与 Rust 作业，增加缓存、并发取消、lint/test/build/一致性检查。

### T7 — 文档与需求基线

- 重写根 README 和 `Docs/00`，达到 SmartPort 的可导航详细度：环境、命令、产物、运行方式、架构图、目录、模块表、当前状态、阅读路径。
- 扩展 `Docs/01` 为可执行的任务分级、跨界流程、完成定义、命名和安全规范。
- 将 `Docs/02` 拆成有编号、验收条件和真实状态的 M0/M1 清单；仅框架落地项可标完成，接口占位不冒充业务完成。
- 扩展 `Docs/03`，记录工具版本、依赖方向、构建入口和暂缓技术决策。
- 新增 `Docs/04-AI辅助开发指南.md`，说明最小上下文读取与按范围验证矩阵。

## 5. 明确不包含

- 不接入 OpenAI、Anthropic、国产模型或任何真实 Provider。
- 不读取邮箱、问题单、Git 远端、BIOS/BMC 生产日志或企业账号。
- 不选择或接入 RAG 向量库、消息队列、数据库、浏览器自动化、插件沙箱。
- 不实现邮件发送、提单、push/merge、电子流等外部副作用。
- 不把 Onboarding、Provider 路由、SessionStart 或 Tool 执行勾选为完整业务能力；本次只交付它们的工程接口与安全骨架。
- 不发布、不提交远端、不创建 PR；这些动作如有需要需另行确认。

## 6. 验收

实施完成后至少执行并记录：

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

若本机缺少 Node/Rust 或系统依赖，必须明确记录缺失工具与未执行项；不能把“未运行”写成“通过”。如工具可通过工作区已提供的运行时使用，则优先使用该运行时完成验证。

完成标准：

- 目录、文档、构建脚本和 CI 相互一致；
- Desktop 和 Rust workspace 可构建；
- 安全边界有自动测试；
- 无密钥、生产样本、生成物和伪造业务数据；
- 本 Plan 末尾追加真实命令结果、限制和回滚说明。

## 7. 回滚方式

本次仅修改仓库内框架和文档，不迁移生产数据。回滚时可按 T1–T7 的独立变更组撤销；CoreEngine 公共导出保持兼容，Desktop 入口仍为 `App.vue`，因此任一模块化步骤均可单独回退。锁文件、构建脚本和 CI 必须作为一个工具链变更组共同回滚，避免版本漂移。

## 8. 待用户确认

用户已于 2026-07-15 明确确认，按 T1 → T7 实施。

## 9. 执行结果

### 已完成范围

- T1：根与三个运行时局部 AGENTS、`CONTRIBUTING.md`、PR/Issue 模板已落地；未虚构 CODEOWNERS。
- T2：Desktop 已拆分为 Layout、Conversation、TaskFlow、Harness、Shared，并补齐模块 README、ESLint 与一致性入口。
- T3：CoreEngine 已拆分 Workflow、Policy、Provider、Tool、Harness；保留原 Workflow 根导出兼容性。
- T4：DesktopHost 增加只读 `runtime_status=unconfigured` 适配，WorkspaceService 增加全部为 false 的能力声明和测试。
- T5：四个 Skills 使用统一章节，Hooks 增加 schema/默认禁用示例，Memory/References 增加生命周期和敏感规则。
- T6：提交 pnpm/Cargo 锁文件规则、Makefile、PowerShell、CI、缓存和一致性检查；补齐 Tauri 强制需要的可重建 SVG/PNG 占位图标。
- T7：README 与 Docs 00–04 已按环境、命令、产物、架构、模块、现状和阅读路径细化；任务项区分骨架与真实接入。

### 验证证据

2026-07-15 实际执行：

```text
pnpm 11.12.0 install --frozen-lockfile
结果：通过，按 Desktop/pnpm-lock.yaml 安装 188 个 package；仅 esbuild 执行允许的安装脚本。

make check
结果：通过。
- Desktop: vue-tsc、Vite build（26 modules）、ESLint 0 warning、工程一致性检查通过
- Rust: cargo build 通过、cargo fmt --check 通过、cargo clippy -D warnings 通过
- Tests: CoreEngine 10 + WorkspaceService 1，共 11 passed / 0 failed

cargo run -p copiner-workspace-service
结果：输出 "unconfigured; no listener or connector started"，未监听端口、未启动连接器。

浏览器验收 http://127.0.0.1:5173/
- 1280px：三栏显示，无横向溢出
- 700px：导航侧栏与 TaskInspector 按断点隐藏，无横向溢出
- Console：0 warning / 0 error

Tauri icon：icon.png 为 512×512 RGBA PNG，Cargo build 不再因缺失默认图标失败。
```

最终另执行 `git diff --check`，结果通过。

### 限制

- 当前机器没有 `pwsh`，因此 `build.ps1` 由工程一致性脚本核对关键目标，但未在 Windows/PowerShell 实机执行。
- 未启动完整 Tauri GUI；DesktopHost 已通过 cargo build/test/clippy，WebView 已通过本地浏览器宽/窄屏验收。
- Provider、Tool 执行器、SessionStart 加载器、Onboarding、RAG 和企业连接器仍按 `Docs/02-任务清单.md` 保持未完成。

### 回滚

未迁移生产数据、未调用企业系统、未创建外部副作用。可按 T1–T7 独立回滚；锁文件、构建脚本、CI 和一致性检查应作为同一工具链组回滚。
