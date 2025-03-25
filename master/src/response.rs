use std::fmt::Display;

use serde::Serialize;

/// 响应状态码枚举
#[derive(Debug, Clone, Copy, Serialize)]
pub enum StatusCode {
  /// 成功
  Success = 0,
  /// 参数错误
  ParamError = 1000,
  /// 认证失败
  AuthFailed = 1001,
  /// 权限不足
  Forbidden = 1002,
  /// 数据验证失败
  ValidationFailed = 1003,
  /// 服务器内部错误
  ServerError = 2000,
  /// 数据库错误
  DbError = 2001,
  /// 哈希密码错误
  HashPasswordError = 2002,
  /// 未实现的功能
  NotImplemented = 3000,
  /// 用户不存在
  UserNotFound = 3001,
  /// 用户已存在
  UserExist = 3002,
  /// 密码错误
  PasswordError = 3003,
  /// Agent 已存在
  AgentExist = 3004,
  /// Agent 不存在
  AgentNotFound = 3005,
  /// Deployment 不存在
  DeploymentNotFound = 3006,
  /// 发送 Agent 请求失败
  SendAgentRequestError = 3007,
  /// Agent 认证失败
  AgentAuthFailed = 3008,
}

impl StatusCode {
  pub fn message(&self) -> &str {
    match self {
      StatusCode::Success => "操作成功",
      StatusCode::ParamError => "参数错误",
      StatusCode::AuthFailed => "认证失败",
      StatusCode::Forbidden => "权限不足",
      StatusCode::ValidationFailed => "数据验证失败",
      StatusCode::ServerError => "服务器内部错误",
      StatusCode::DbError => "数据库错误",
      StatusCode::NotImplemented => "未实现的功能",
      StatusCode::UserExist => "用户已存在",
      StatusCode::PasswordError => "密码错误",
      StatusCode::HashPasswordError => "生成哈希密码错误",
      StatusCode::AgentNotFound => "找不到 Agent",
      StatusCode::SendAgentRequestError => "发送 Agent 请求失败",
      StatusCode::AgentExist => "Agent 已存在",
      StatusCode::DeploymentNotFound => "Deployment 不存在",
      StatusCode::UserNotFound => "用户不存在",
      StatusCode::AgentAuthFailed => "Agent 认证失败",
    }
  }
}

#[derive(Debug, Serialize)]
pub struct Response<T> {
  pub code: i32,
  pub msg: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
}

impl<T> Response<T> {
  pub fn success(data: T) -> Self {
    Response {
      code: StatusCode::Success as i32,
      msg: StatusCode::Success.message().to_string(),
      data: Some(data),
    }
  }

  pub fn error(code: StatusCode) -> Self {
    Response {
      code: code as i32,
      msg: code.message().to_string(),
      data: None,
    }
  }
}

impl<T> Display for Response<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, r#"{{ "code": {}, "msg": "{}" }}"#, self.code, self.msg)
  }
}
