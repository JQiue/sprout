use std::fmt::Display;

use serde::Serialize;

/// 响应状态码枚举
#[derive(Debug, Clone, Copy, Serialize)]
pub enum StatusCode {
  Success = 0, // 成功
  // 客户端错误 (1000-1999)
  /// 参数错误
  ParamError = 1000,
  // 认证失败
  AuthFailed = 1001,
  // 权限不足
  Forbidden = 1002,
  // 资源不存在
  NotFound = 1003,
  // 数据验证失败
  ValidationFailed = 1004,
  // 请求频率限制
  RequestLimit = 1005,
  /// 服务端错误 (2000-2999)
  // 服务器内部错误
  ServerError = 2000,
  // 数据库错误
  DbError = 2001,
  // 哈希密码错误
  HashPasswordError = 2002,
  // 业务错误 (3000-3999)
  // 用户不存在
  UserNotExist = 3000,
  // 用户已存在
  UserExist = 3001,
  // 密码错误
  PasswordError = 3002,
  // 余额不足
  BalanceNotEnough = 3004,
  // 第三方服务错误 (4000-4999)
}

impl StatusCode {
  pub fn message(&self) -> &str {
    match self {
      StatusCode::Success => "操作成功",
      StatusCode::ParamError => "参数错误",
      StatusCode::AuthFailed => "认证失败",
      StatusCode::Forbidden => "权限不足",
      StatusCode::NotFound => "资源不存在",
      StatusCode::ValidationFailed => "数据验证失败",
      StatusCode::RequestLimit => "请求过于频繁",
      StatusCode::ServerError => "服务器内部错误",
      StatusCode::DbError => "数据库错误",
      StatusCode::UserNotExist => "用户不存在",
      StatusCode::PasswordError => "密码错误",
      StatusCode::BalanceNotEnough => "余额不足",
      StatusCode::UserExist => "用户已存在",
      StatusCode::HashPasswordError => "生成哈希密码错误",
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

  pub fn error_with_msg(code: StatusCode, msg: String) -> Self {
    Response {
      code: code as i32,
      msg,
      data: None,
    }
  }
}

impl<T> Display for Response<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, r#"{{ "code": {}, "msg": "{}" }}"#, self.code, self.msg)
  }
}
