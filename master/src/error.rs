use actix_web::{HttpResponse, ResponseError};

use crate::response::Response;

#[derive(Debug)]
pub enum AppError {
  Error,
  Env,
  Database,
  Authorization,
  UserNotFound,
  Forbidden,
  HashPasswordError,
  UserExist,
  PasswordError,
  AgentExist,
  AgentNotFound,
  AgentAuthFailed,
  DeploymentNotFound,
  RpcCallError,
  NotImplemented,
}

impl AppError {
  pub fn code(&self) -> i32 {
    match self {
      AppError::Error => 1000,
      AppError::Database => 1001,
      AppError::Env => 1002,
      AppError::Authorization => 1003,
      AppError::Forbidden => 1004,
      AppError::UserNotFound => 1005,
      AppError::HashPasswordError => 1006,
      AppError::NotImplemented => 3000,
      AppError::UserExist => todo!(),
      AppError::PasswordError => todo!(),
      AppError::AgentExist => todo!(),
      AppError::AgentNotFound => 2001,
      AppError::DeploymentNotFound => todo!(),
      AppError::AgentAuthFailed => todo!(),
      AppError::RpcCallError => 3001,
    }
  }
  pub fn message(&self) -> String {
    match self {
      AppError::Error => "err".to_string(),
      AppError::Env => "Env".to_string(),
      AppError::Forbidden => "".to_string(),
      AppError::Authorization => "Authorization".to_string(),
      AppError::Database => "Database".to_string(),
      AppError::HashPasswordError => "".to_string(),
      AppError::NotImplemented => "".to_string(),
      AppError::UserNotFound => "".to_string(),
      AppError::UserExist => "".to_string(),
      AppError::PasswordError => "".to_string(),
      AppError::AgentExist => "".to_string(),
      AppError::AgentNotFound => "Agent not found".to_string(),
      AppError::DeploymentNotFound => "".to_string(),
      AppError::AgentAuthFailed => "".to_string(),
      AppError::RpcCallError => "Rpc call error".to_string(),
    }
  }
}

impl ResponseError for AppError {
  fn error_response(&self) -> HttpResponse {
    tracing::error!("{:#?}", self.message());
    HttpResponse::Ok().json(Response::<()> {
      data: None,
      code: self.code(),
      msg: self.message(),
    })
  }
}

impl std::fmt::Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message())
  }
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<envy::Error> for AppError {
  fn from(err: envy::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Env
  }
}

impl From<helpers::jwt::Error> for AppError {
  fn from(err: helpers::jwt::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<actix_web::http::header::ToStrError> for AppError {
  fn from(err: actix_web::http::header::ToStrError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}

impl From<reqwest::Error> for AppError {
  fn from(err: reqwest::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::RpcCallError
  }
}

impl From<sea_orm::DbErr> for AppError {
  fn from(err: sea_orm::DbErr) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Database
  }
}

impl From<helpers::hash::Error> for AppError {
  fn from(err: helpers::hash::Error) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}
