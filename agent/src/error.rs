use actix_web::{HttpResponse, ResponseError, http::StatusCode};

#[derive(Debug)]
pub enum AppError {
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

impl AppError {
  pub fn code(&self) -> i32 {
    match self {
      AppError::ParamError => 1000,
      AppError::DbError => 1001,
      AppError::HashPasswordError => 1002,
      AppError::NotImplemented => 3000,
      AppError::AuthFailed => todo!(),
      AppError::Forbidden => todo!(),
      AppError::ValidationFailed => todo!(),
      AppError::ServerError => todo!(),
      AppError::UserNotFound => todo!(),
      AppError::UserExist => todo!(),
      AppError::PasswordError => todo!(),
      AppError::AgentExist => todo!(),
      AppError::AgentNotFound => todo!(),
      AppError::DeploymentNotFound => todo!(),
      AppError::SendAgentRequestError => todo!(),
      AppError::AgentAuthFailed => todo!(),
    }
  }
  pub fn message(&self) -> String {
    match self {
      AppError::ParamError => "参数错误".to_string(),
      AppError::AuthFailed => "认证失败".to_string(),
      AppError::Forbidden => "".to_string(),
      AppError::ValidationFailed => "".to_string(),
      AppError::ServerError => "".to_string(),
      AppError::DbError => "".to_string(),
      AppError::HashPasswordError => "".to_string(),
      AppError::NotImplemented => "".to_string(),
      AppError::UserNotFound => "".to_string(),
      AppError::UserExist => "".to_string(),
      AppError::PasswordError => "".to_string(),
      AppError::AgentExist => "".to_string(),
      AppError::AgentNotFound => "".to_string(),
      AppError::DeploymentNotFound => "".to_string(),
      AppError::SendAgentRequestError => "".to_string(),
      AppError::AgentAuthFailed => "".to_string(),
    }
  }
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::ServerError
  }
}

impl From<envy::Error> for AppError {
  fn from(err: envy::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::ServerError
  }
}

impl From<helpers::jwt::Error> for AppError {
  fn from(err: helpers::jwt::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::UploadTokenError
  }
}

impl From<actix_web::http::header::ToStrError> for AppError {
  fn from(err: actix_web::http::header::ToStrError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::ServerError
  }
}

impl From<reqwest::Error> for AppError {
  fn from(err: reqwest::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::RpcCallError
  }
}

impl ResponseError for AppError {
  fn status_code(&self) -> StatusCode {
    StatusCode::OK
  }
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code()).body(self.to_string())
  }
}

impl std::fmt::Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message())
  }
}
