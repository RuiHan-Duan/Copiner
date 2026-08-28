# WorkspaceService

云上或企业内网 Workspace 的独立运行时边界，未来承载 RAG、记忆索引、任务队列与企业系统网关。它不是 DesktopHost 的内部模块，也不会被前端直接访问。

## 当前实现

`WorkspaceCapabilities` 显式声明四项能力均为 `false`。二进制启动时只打印 `unconfigured` 状态：

- 不监听端口；
- 不连接数据库、消息队列或向量库；
- 不读取 Memory、References 或企业系统；
- 不提供虚假的健康接口或成功结果。

## 未来模块边界

| 能力 | 责任 | 接入前置条件 |
|---|---|---|
| RAG | 文档切分、检索和来源元数据 | 向量库、部署和敏感策略评审 |
| Memory Index | 可重建索引，不替代纯文本正文 | 生命周期、删除和同步评审 |
| Task Queue | 云上云下任务状态中转 | 幂等、鉴权、冲突和审计协议 |
| Enterprise Gateway | 企业 API 统一门面 | 系统授权、字段契约和网络边界 |

上述能力完成专项评审前保持不可用，不能根据桌面占位字段反推生产接口。

## 验证

执行 Rust workspace 的 fmt、clippy、test 和 build；未来增加网络服务时另补健康检查和实际请求证据。
