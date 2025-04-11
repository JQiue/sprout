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
  UserExist,
  PasswordError,
  AgentExist,
  AgentNotFound,
  AgentAuthFailed,
  DeploymentNotFound,
  RpcCallError,
  NotImplemented,
  SiteNotFound,
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
      AppError::UserExist => 1006,
      AppError::PasswordError => 1007,
      AppError::AgentExist => 2000,
      AppError::AgentNotFound => 2001,
      AppError::AgentAuthFailed => 2002,
      AppError::DeploymentNotFound => 2003,
      AppError::SiteNotFound => 2004,
      AppError::NotImplemented => 3000,
      AppError::RpcCallError => 3001,
    }
  }
  pub fn message(&self) -> String {
    match self {
      AppError::Error => "err".to_string(),
      AppError::Env => "Env".to_string(),
      AppError::Forbidden => "Forbidden".to_string(),
      AppError::Authorization => "Authorization".to_string(),
      AppError::Database => "Database".to_string(),
      AppError::NotImplemented => "Not implemented".to_string(),
      AppError::UserNotFound => "User not found".to_string(),
      AppError::UserExist => "User exist".to_string(),
      AppError::PasswordError => "Password error".to_string(),
      AppError::AgentExist => "Agent exist".to_string(),
      AppError::AgentNotFound => "Agent not found".to_string(),
      AppError::DeploymentNotFound => "Deployment not found".to_string(),
      AppError::AgentAuthFailed => "Agent auth Failed".to_string(),
      AppError::RpcCallError => "Rpc call error".to_string(),
      AppError::SiteNotFound => "Site not found".to_string(),
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
    tracing::error!("{:#?}", err.kind());
    Self::Authorization
  }
}

impl From<actix_web::http::header::ToStrError> for AppError {
  fn from(err: actix_web::http::header::ToStrError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
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

impl From<rpc::error::AppError> for AppError {
  fn from(err: rpc::error::AppError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::RpcCallError
  }
}

impl From<std::net::AddrParseError> for AppError {
  fn from(err: std::net::AddrParseError) -> Self {
    tracing::error!("{:#?}", err);
    AppError::Error
  }
}
