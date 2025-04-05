# Sprout

## 系统架构

用户端：cli deploy -> 创建部署信息 -> 节点集群

游客端：域名 -> nginx -> 节点集群

访问流程

1. 用户将自己的域名 (cname.com) CNAME 到系统提供的域名 (example1.com)。
2. 用户访问 cname.com。
3. DNS 服务器将 cname.com 解析到 example1.com。
4. 用户的请求到达 example1.com 对应的 Nginx 负载均衡器。
5. Nginx 负载均衡器将请求转发到后端的某个节点 (node1)。
6. 节点 (node1) 处理请求，并将结果返回给用户。

### 1. Agent 功能

- Agent 注册与心跳
  - 启动时向 Master 注册并提供服务器信息
  - 定期发送心跳包维持连接
  - 实时报告服务器状态

- 任务处理能力
  - 接收并执行部署任务
  - 接收并执行监控任务
  - 及时反馈任务执行结果

### 2. 核心业务流程

- 部署流程
  - Master API 接收部署请求
  - 创建部署记录并分发任务
  - Agent 执行具体部署步骤
    - 获取部署文件
    - 执行构建流程
    - 配置 Nginx 服务
    - 更新部署状态
    - 同步站点信息
- 监控与反馈
  - 实时监控 Agent 状态与任务执行结果
  - 异常情况处理机制
  - 部署状态追踪

### 3. 用户交互与监控管理

- 用户交互
  - Web 管理界面
  - RESTful API 接口
  - 完整的部署管理功能

- 日志与监控
  - 完整的部署日志记录
  - 系统监控告警机制
  - 实时监控 Agent 状态
  - 异常情况处理机制
  - 部署状态追踪

### 4. 安全保障

- 通信安全
  - HTTPS 加密传输
  - Token 身份认证
- 系统安全
  - 防火墙防护
  - 数据定期备份
  - 系统恢复能力

### 5. 性能优化与架构扩展

- 性能优化
  - 异步任务处理
  - 缓存加速
  - CDN 支持
  - 负载均衡

- 架构扩展
  - Docker 容器化
  - Kubernetes 编排
  - 微服务架构
  - 消息队列
  - 数据分片

## Master API

| 路由                                  | 说明                  |
| ------------------------------------- | --------------------- |
| `POST /api/user`                      | 注册用户              |
| `POST /api/user/token`                | 获取 jwt              |
| `GET /api/user/info`                  | 获取用户信息          |
| `POST /api/user/password`             | 修改用户密码          |
| `POST /api/token/refresh`             | 刷新 jwt 时间         |
| `POST /api/site`                      | 创建 Site             |
| `DELETE /api/site`                    | 删除 Site             |
| `GET /api/deployment/{deployment_id}` | 获取部署信息          |
| `POST /api/deployment`                | 创建部署信息          |
| `POST /api/deployment/status`         | 更新部署信息          |
| `POST /api/agent`                     | 创建 Agent            |
| `GET /api/agent/{agent_id}`           | 获取 Agent 的系统状态 |
| `POST /api/{agent_id}/token`          | 刷新 Agent 的 token   |

## Agent API

| 路由                    | 说明                         |
| ----------------------- | ---------------------------- |
| `GET /api/heartbeat`    | 返回 Agent 的状态            |
| `POST /api/upload/init` | 生成上传 token，包含 site_id |
| `POST /api/upload/file` | 上传网页文件                 |
| ``                      |                              |
