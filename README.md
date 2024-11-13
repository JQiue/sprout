# Sprout

## 工作流程

1. Agent 启动并注册：
  + Agent 启动时向 Master 注册
  + 提供服务器信息和状态
  + 定期发送心跳包
2. 创建部署：
  + 用户通过 Master 的 API 创建部署
  + Master 创建部署记录
  + Master 将部署任务分发给对应的 Agent
3. Agent 执行部署：
  + 接收部署请求
  + 下载或接收部署文件
  + 执行构建步骤
  + 配置 Nginx
  + 报告部署结果
4. 监控和管理：
  + Master 监控所有 Agent 的状态
  + 处理部署失败的情况
  + 提供部署状态查询
