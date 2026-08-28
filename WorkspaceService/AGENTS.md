# WorkspaceService Agent 指令

继承根 `AGENTS.md`，修改前先读本目录 `README.md`。

## 边界

- WorkspaceService 是未来云上/内网 Workspace 的独立运行时边界，不是 DesktopHost 的内部模块。
- 当前只允许表达能力声明和未配置状态，不监听端口、不连接数据库、不读取企业数据。
- RAG、记忆索引、任务队列、同步协议和企业系统网关都尚未选型；不得自行引入产品或伪实现。
- 任何网络接口、鉴权、存储、端口和部署变化都需专项 Plan 与安全评审。

## 验证

执行 Rust workspace 的 fmt、clippy、test 和 build；若未来增加服务端点，还必须补健康检查与实际请求证据。
