# Copiner UI 与品牌图标重设计 Plan

> 状态：已完成
> 日期：2026-08-27
> 前置成果：`2026-08-27-Local-Harness-Visible-Loop.md`

## 1. 目标

在不改变 CoreEngine、DesktopHost 命令和 Harness 安全语义的前提下，重做 Copiner 的视觉识别与桌面交互层，使当前可运行成果从“工程骨架界面”升级为有独立品牌性、层级清晰、可用于后续能力扩展的产品界面。

本次采用 **Copiner 字标系统**：不从项目架构隐喻或未经确认的名称词源推导品牌含义，直接从 `Copiner` 的字母形态建立识别。

- 主标识：定制大写 `C`，通过非对称收笔和内侧短笔形成独有轮廓；
- 完整字标：`Copiner` wordmark，首字母标识与正文保持同一笔画比例；
- UI 延展：分隔线、阶段轨道、选中态和按钮切角沿用 C 标识的收笔角度；
- 视觉气质：瓷白背景、纯白工作面、深蓝灰文字与克制的钴蓝/蓝紫主色；卡片和浮层使用低透明度浅蓝阴影，避免把名称误读为 Coin 而继续使用金币色，也避免机器人头像和通用 SaaS 彩色渐变。

## 2. 用户可见成果

1. 新应用图标：定制 `C` 字母标在 Dock、窗口和侧栏中保持同一识别；
2. 新桌面壳：品牌区、任务区、运行控制区有清晰主次，不再像三列后台管理模板；
3. 新空态：首页直接解释“本地 Harness、未接模型、如何开始”，任务输入成为主要操作；
4. 新运行态：任务标题、阶段轨道、事件时间线和批准动作形成一个连续工作面；
5. 宽屏、普通笔记本和窄屏均可使用，键盘焦点、对比度和 reduced-motion 可验证。

## 3. Todos 与改动边界

### T1 — 品牌图标系统

范围：`Crates/DesktopHost/icons/`、Desktop 品牌展示组件。

- 重新绘制 512×512 SVG：浅色底上的蓝色定制 `C` 为唯一主体，保证 16px 小尺寸仍能识别；
- 不加入圆、方、孔等架构概念，不使用 Coin、硬币、机器人、对话气泡或火花隐喻；
- 使用纯矢量路径和有限色板，不使用小字号文字、阴影噪点或生成式位图；
- 从 SVG 重建 512×512 RGBA PNG，并验证透明度、尺寸与 Tauri build；
- 新增可复用的 Vue 品牌标记组件，侧栏和空态使用同一几何语言。

不包含：商标注册、完整品牌手册、多平台安装包图标矩阵或营销插画。

### T2 — Desktop 信息架构与组件重排

范围：`Desktop/src/Layout/`、`Conversation/`、`TaskFlow/`、`Harness/`、`Shared/`。

- 侧栏改为紧凑的产品 rail：品牌、当前工作区、导航和本地状态；
- Header 改为任务上下文与阶段轨道，不再让阶段胶囊抢占全部视觉焦点；
- 空态改为以 Copiner wordmark 和任务输入为中心的起始工作面，弱化技术演示感；
- 运行态改为“Run Canvas”：顶部任务摘要、中部事件轨道、完成结果卡；
- TaskInspector 改为“Run Control”：当前动作、批准边界、Todo 进度和安全说明；
- 保留当前组件输入输出和 Tauri adapter，不复制或改变 Workflow 规则。

### T3 — 视觉令牌、响应式与可访问性

- 重建浅色 CSS 令牌：瓷白背景、纯白表面、深蓝灰文字、钴蓝强调、成功/错误状态、浅蓝边框与柔和阴影；
- 阴影使用浅蓝而非中性黑，但重要区域同时保留边框或背景差，不能只靠阴影区分；
- 使用系统字体栈，不增加字体下载或 UI 框架依赖；
- 交互控件提供 hover、active、disabled、focus-visible；
- 1024px 以下重新组织 Run Control，720px 以下切为单列，并保留所有推进按钮；
- 对 `prefers-reduced-motion` 禁用非必要动效；图标与状态不只依赖颜色表达。

### T4 — 文档与验收同步

- 同步 `Desktop/README.md`、Conversation/TaskFlow/Harness README 与 `Crates/DesktopHost/README.md`；
- 在 `Docs/00-项目介绍.md` 和 `Docs/02-任务清单.md` 记录 UI 重设计为 M1-06 的可见进展，但不将该里程碑标记完成；
- 本 Plan 追加真实验证结果与限制。

## 4. 明确不包含

- 不修改 CoreEngine Workflow、Provider、Tool 或审批契约；
- 不新增模型、假对话、模拟指标或企业数据；
- 不接入 Tailwind、组件库、图标库、动画库或远程字体；
- 不实现设置页、主题切换、持久化、多会话或桌宠；
- 不 commit、push、创建 PR 或发布。

## 5. 验收

自动验证：

```text
pnpm 11.12.0 frozen install
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

视觉与交互验证：

- 浏览器 1440×900、1024×768、700×900：无页面级横向溢出，输入、阶段和 Run Control 均可见；
- 浏览器与 Tauri 均为浅色界面；卡片阴影呈浅蓝色，无残留深色主题表面；
- console 0 warning / 0 error；
- Tab 顺序、focus-visible、按钮状态和文本对比度人工检查；
- `make dev` 实际启动 Tauri，窗口使用新图标并显示新 UI；
- SVG、PNG 均为 512×512，PNG 为 RGBA。

## 6. 回滚

本次无数据迁移。Desktop 组件/CSS、Vue 品牌标记、SVG/PNG 图标和文档可作为一个视觉变更组整体回滚；Harness 运行契约与 Rust 领域代码不受影响。

## 7. 待确认

用户已于 2026-08-27 确认进入 Execute。授权范围仅包含本地 UI、图标、文档修改和本地验证，不包含发布或外部副作用。

## 8. 执行结果（2026-08-28）

已完成：

- 以定制大写 `C` 建立 Copiner 字标，SVG 与 Desktop `BrandMark.vue` 使用同一几何；
- 图标底板采用带透明外缘的 macOS 连续圆角轮廓，PNG 改由 Sharp 直接栅格化 SVG，避免 Quick Look 将透明区压成白色；已验证 512×512、8-bit RGBA 且四角像素 Alpha 为 0；
- Desktop 已重排为浅色 Run Canvas、响应式导航、阶段轨道和 Run Control；
- 主题使用瓷白画布、白色表面、深蓝灰文字、钴蓝强调和浅蓝阴影；
- 无模型、无 Tool、无企业系统和浏览器只读预览等空态保持真实；
- 模块 README、项目介绍和 M1-06 进度已同步。

自动验证：

```text
corepack pnpm@11.12.0 --dir Desktop install --frozen-lockfile --store-dir /private/tmp/copiner-pnpm-store-20260827  PASS
corepack pnpm@11.12.0 --dir Desktop run typecheck                                                  PASS
corepack pnpm@11.12.0 --dir Desktop run build                                                      PASS
corepack pnpm@11.12.0 --dir Desktop run lint                                                       PASS
corepack pnpm@11.12.0 --dir Desktop run check:consistency                                          PASS
cargo fmt --all -- --check                                                                         PASS
cargo clippy --workspace --all-targets -- -D warnings                                              PASS
cargo test --workspace                                                                             PASS (16 tests)
cargo build --workspace                                                                            PASS
git diff --check                                                                                    PASS（逐个变更路径复核；全量调用在本机 Git 进程中无输出挂起）
```

视觉与运行验收：

- 浏览器 1440×900：三栏完整，页面 `scrollWidth = 1440`，无横向溢出；
- 浏览器 1024×768：导航收窄为 88px，Run Control 保留，页面 `scrollWidth = 1024`；
- 浏览器 700×900：切换单列，工作区和 Run Control 均为 700px，无横向溢出；
- 浏览器 console 只有 Vite debug 连接日志，0 warning / 0 error；
- `focus-visible` 对按钮和输入提供 2px 钴蓝轮廓与浅蓝焦点阴影；
- `make dev` 实际启动 `copiner-desktop-host`，Tauri 窗口 1224×780 下无内容裁切；实机截图为 `/private/tmp/copiner-tauri-redesign.png`；
- macOS 未授予辅助功能控制，因此未在 Tauri 窗口内自动点击完整闭环；状态迁移由 DesktopHost/CoreEngine 的 16 项自动测试覆盖。

未执行 commit、push、PR 或发布；Tauri 开发窗口保留运行，便于人工查看。
