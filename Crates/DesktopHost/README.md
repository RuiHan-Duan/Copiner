# DesktopHost

Copiner 的 Tauri 2 本地宿主，负责窗口生命周期、WebView 资源和桌面命令适配。

## 依赖边界

DesktopHost 可以依赖 `CoreEngine`，但不在命令处理器中复制工作流、安全路由和审批判断。真实 Provider、密钥链、文件系统或企业连接器接入前必须经过单独技术与安全 Plan。

## 当前实现

- 启动 Tauri 窗口并加载 `Desktop/dist/`；
- 暴露 `runtime_status` 与本地 Harness 的创建、snapshot、推进、重置命令；
- 以进程内 managed state 保存单个 `WorkflowSession`，所有迁移仍由 CoreEngine 校验；
- 命令只返回结构化 UI DTO，错误不记录任务正文；
- 不监听额外端口，不连接模型或企业系统；
- bundling 仍关闭，当前只验收开发与构建骨架；
- `icons/icon.svg` 是可审阅的 Copiner 定制 `C` 字标源图；底板使用带透明外缘的 macOS 连续圆角轮廓，避免 Dock 显示成满画布白色方块。`icon.png` 是由同一矢量源生成的 512×512 RGBA 构建资产；当前未包含安装包所需的多平台图标矩阵。

本地 Harness 会话不持久化，应用重启后清空；它只验证 Desktop → DesktopHost → CoreEngine 纵向闭环，不代表 Provider 或 Tool 执行器已经接入。

## 运行

仓库根执行 `make dev`。前端 dev server 固定 `5173`，配置见 `tauri.conf.json`。
