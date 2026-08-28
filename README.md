# Copiner — 本地优先企业 AI 协作 Agent

Copiner 面向企业内部的非代码工作。员工只与一个 Agent 对话，由 Harness、CoreEngine 和经授权的工具去协调文档、邮件、问题单、BIOS/BMC 资料及其他系统。

当前仓库是 **M0/M1 可运行工程骨架**：闭环工作流、安全策略、首个本地 Harness 可见闭环、Harness 手册和 Workspace 边界已经成形，但尚未接入真实 LLM、RAG、邮箱、问题单、浏览器自动化或企业数据。

## 当前能验证什么

- 工作流强制经过 Clarify → Todos → Plan → Approval → Execute → Feynman → Archive；
- Unknown/Internal/Sensitive 数据不能路由到 External Provider；
- 本地或外部写操作在动作级批准前不能执行；
- Desktop 可通过 Tauri 创建并推进单个内存 Workflow，会显示事件与完成摘要；
- Desktop 始终明确显示“未连接模型”，不生成假对话或业务指标；
- Desktop 与 Rust workspace 有统一 build/test/lint/CI 入口；
- Skills、Hooks、Memory、References 使用可审阅的本地文本边界。

以上是框架能力，不代表已存在可用模型、工具执行器或企业连接器。

## 环境准备

| 工具 | 版本 | 用途 |
|---|---|---|
| Node.js | 24.x（CI 24.14.0） | Desktop 前端工具链 |
| pnpm | 11.12.0 | 唯一前端包管理器 |
| Rust | stable + rustfmt + clippy | CoreEngine、DesktopHost、WorkspaceService |
| Tauri 系统依赖 | Tauri 2 官方要求 | 完整桌面开发/打包 |
| PowerShell | 7.x，可选 | Windows 统一入口 |

不提交 `.env`、Token 或证书。未来 Provider Key 只能从环境或系统密钥链读取。

## 构建与验证

在仓库根目录：

```bash
make install      # 按锁文件安装 Desktop 依赖
make build        # Desktop + Rust workspace
make test         # Desktop 工程检查 + Rust 单元测试
make lint         # ESLint/一致性 + rustfmt/clippy
make check        # build + test + lint
make clean        # 清理生成物
```

Windows 未安装 make 时，用 PowerShell 7 执行等价命令：

```powershell
pwsh -File .\build.ps1 install
pwsh -File .\build.ps1 check
```

也可以使用分栈原生命令，见 `Desktop/README.md`、`Crates/AGENTS.md` 和 `WorkspaceService/README.md`。

## 编译产物

| 边界 | 产物 | 位置 |
|---|---|---|
| Desktop | WebView 静态资源 | `Desktop/dist/` |
| CoreEngine | Rust library | `target/` |
| DesktopHost | Tauri 本地宿主 binary | `target/` |
| WorkspaceService | 未配置服务骨架 binary | `target/` |

`dist/`、`target/`、`node_modules/` 均不入库。

## 运行

```bash
make dev             # 完整 Tauri 桌面调试
make workspace-run   # 只验证 WorkspaceService 未配置状态
```

仅开发 WebView：

```bash
cd Desktop
pnpm run dev         # http://localhost:5173
pnpm run preview     # 构建后 http://localhost:4173
```

`Desktop/dist/index.html` 必须由 Tauri 或 HTTP server 提供，不建议用 `file://` 直接打开。WorkspaceService 当前不监听端口。

## 总体架构

```text
┌─────────────────────────────────────────────────────────────┐
│ Desktop/  Vue 3 + TypeScript                                │
│ Layout · Conversation · TaskFlow · Harness · Shared         │
└───────────────────────────┬─────────────────────────────────┘
                            │ Tauri commands（本地 snapshot 已接入；events 待接入）
┌───────────────────────────▼─────────────────────────────────┐
│ Crates/DesktopHost  本地宿主与适配                           │
└───────────────────────────┬─────────────────────────────────┘
                            │ Rust API
┌───────────────────────────▼─────────────────────────────────┐
│ Crates/CoreEngine  领域内核                                 │
│ Workflow · Policy · Provider · Tool · Harness               │
└───────────────┬───────────────────────────────┬─────────────┘
                │ 本地/内网 Provider（待接入）     │ 同步协议（待评审）
┌───────────────▼──────────────┐  ┌─────────────▼─────────────┐
│ Skills · Hooks · Memory      │  │ WorkspaceService          │
│ References（可移植文本边界）   │  │ RAG/索引/队列/网关均未配置   │
└──────────────────────────────┘  └───────────────────────────┘
```

依赖方向只有 Desktop → DesktopHost → CoreEngine。WorkspaceService 是独立运行时，不反向依赖桌面 UI。Provider、Tool 和 Skill 通过接口或注册表扩展，厂商细节不得写入 Workflow。

## 目录结构

```text
Copiner/
├── AGENTS.md / COWORK.md       开发 Agent 与产品内 Agent 行为入口
├── Makefile / build.ps1        跨平台统一构建入口
├── Scripts/                    工程一致性检查
├── Desktop/                    Vue 3 WebView 与模块说明
├── Crates/
│   ├── CoreEngine/             工作流、路由、工具与 Harness 领域规则
│   └── DesktopHost/            Tauri 本地宿主
├── WorkspaceService/           云上/内网 Workspace 独立边界
├── Skills/                     Mail/Ticket/BiosBmc/CodeSync 手册
├── Hooks/                      SessionStart schema 与禁用示例
├── Memory/                     可移植纯文本记忆规范
├── References/                 Skill 按需加载的重资料入口
├── Docs/                       需求、规范、架构与技术决策
├── Plans/                      跨界变更 Plan 与验收记录
└── .github/                    CI、PR 与 Issue 模板
```

模块与代码映射见 `Docs/00-项目介绍.md`。

## 当前状态与限制

| 能力 | 状态 |
|---|---|
| 闭环 Workflow | 已实现并有单元测试 |
| Provider 接口/敏感策略 | 骨架已实现；无真实 Provider、成本/降级逻辑 |
| Tool 注册/动作批准 | 注册与安全判断已实现；无执行器和审计存储 |
| Desktop | 可运行单会话 Harness；无模型对话、持久化和事件订阅 |
| Harness | 手册/schema 已存在；无动态加载器 |
| WorkspaceService | 能力全部显式为 false；无网络服务 |
| Onboarding、RAG、企业系统、桌宠 | 未实现 |

真实进度只认 `Docs/02-任务清单.md`。

## 从哪读起

| 想了解 | 入口 |
|---|---|
| 产品定位、架构和模块 | `Docs/00-项目介绍.md` |
| 开始修改代码 | `Docs/01-开发规范.md` + 目标目录 `AGENTS.md` |
| 查范围和进度 | `Docs/02-任务清单.md` |
| 查技术版本和暂缓决策 | `Docs/03-技术选型.md` |
| 让编码 Agent 安全协作 | `Docs/04-AI辅助开发指南.md` |
| 查产品内 Agent 行为 | `COWORK.md` + 对应 `Skills/*/SKILL.md` |
| 团队分支和提交约定 | `CONTRIBUTING.md` |
