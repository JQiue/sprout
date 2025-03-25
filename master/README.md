# Master

## API

| 路由                                  | 作用                  |
| ------------------------------------- | --------------------- |
| `POST /api/user`                      | 注册用户              |
| `POST /api/user/token`                | 获取 jwt              |
| `GET /api/user/info`                  | 获取用户信息          |
| `POST /api/user/password`             | 修改用户密码          |
| `POST /api/token/refresh`             | 刷新 jwt 时间         |
| `POST /api/site`                      | 创建 Site             |
| `DELETE /api/site`                    | 删除 Site             |
| `GET /api/deployment/{deployment_id}` | 获取部署信息          |
| `POST /api/deployment/status`         | 修改部署信息          |
| `POST /api/agent`                     | 创建 Agent            |
| `GET /api/agent/{agent_id}`           | 获取 Agent 的系统状态 |
| `POST /api/{agent_id}/token`          | 刷新 Agent 的 token   |
