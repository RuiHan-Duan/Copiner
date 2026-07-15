# DesktopHost

Copiner 的 Tauri 2 本地宿主，负责窗口生命周期、WebView 资源和桌面命令适配。

## 依赖边界

DesktopHost 可以依赖 `CoreEngine`，但不在命令处理器中复制工作流、安全路由和审批判断。真实 Provider、密钥链、文件系统或企业连接器接入前必须经过单独技术与安全 Plan。

## 当前实现

- 启动 Tauri 窗口并加载 `Desktop/dist/`；
- 暴露只读 `runtime_status` 命令，明确返回 `unconfigured`；
- 不监听额外端口，不连接模型或企业系统；
- bundling 仍关闭，当前只验收开发与构建骨架；
- `icons/icon.svg` 是可审阅的几何占位源图，`icon.png` 只满足 Tauri 构建要求，不是最终品牌资产。

## 运行

仓库根执行 `make dev`。前端 dev server 固定 `5173`，配置见 `tauri.conf.json`。
