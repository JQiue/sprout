use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use common::Response;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  // 500
  #[error("Internal server error")]
  InternalServerError {
    #[from]
    source: std::io::Error,
  },
  #[error("Environment variable error")]
  Env {
    #[from]
    source: envy::Error,
  },
  #[error("Database error")]
  Database {
    #[from]
    source: sea_orm::DbErr,
  },
  #[error("Authorization error")]
  Authorization,
  #[error("User not found")]
  UserNotFound,
  #[error("Forbidden")]
  Forbidden,
  #[error("User already exists")]
  UserExists,
  #[error("Password error")]
  PasswordError,
  #[error("Agent already exists")]
  AgentExists,
  #[error("Agent not found")]
  AgentNotFound,
  #[error("Deployment not found")]
  DeploymentNotFound,
  #[error("RPC call error: {source}")]
  RpcCallError {
    #[from]
    source: rpc::error::Error,
  },
  #[error("Not implemented")]
  NotImplemented,
  #[error("Site not found")]
  SiteNotFound,
  #[error("Params error")]
  Params {
    #[from]
    souce: validator::ValidationErrors,
  },
  #[error("To str error")]
  ToStrError {
    #[from]
    source: actix_web::http::header::ToStrError,
  },
  #[error("Hash error")]
  HashError {
    #[from]
    source: helpers::hash::Error,
  },
  #[error("JWT error: {source}")]
  JwtError {
    #[source]
    source: helpers::jwt::Error,
  },
  #[error("InvalidJwtSignature")]
  InvalidJwtSignature,
  #[error("ExpiredSignature")]
  ExpiredSignature,
  #[error("Address paser error")]
  AddrParseError {
    #[from]
    source: std::net::AddrParseError,
  },
  #[error("Other error: {message}")]
  Other {
    message: String,
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
  },
}

impl AppError {
  pub fn code(&self) -> i32 {
    match self {
      AppError::InternalServerError { .. }
      | AppError::Database { .. }
      | AppError::Env { .. }
      | AppError::RpcCallError { .. }
      | AppError::JwtError { .. }
      | AppError::Other { .. }
      | AppError::ToStrError { .. }
      | AppError::AddrParseError { .. }
      | AppError::HashError { .. } => 1000,
      AppError::ExpiredSignature | AppError::InvalidJwtSignature => 2000,
      AppError::PasswordError => 2001,
      AppError::Authorization => 2002,
      AppError::Forbidden => 2003,
      AppError::Params { .. } => 2004,
      AppError::AgentNotFound
      | AppError::DeploymentNotFound
      | AppError::SiteNotFound
      | AppError::UserNotFound => 2005,
      AppError::UserExists | AppError::AgentExists => 2006,
      AppError::NotImplemented => 9999,
    }
  }

  pub fn status_code(&self) -> StatusCode {
    match self {
      AppError::InternalServerError { .. }
      | AppError::Database { .. }
      | AppError::Env { .. }
      | AppError::RpcCallError { .. }
      | AppError::JwtError { .. }
      | AppError::HashError { .. }
      | AppError::Other { .. }
      | AppError::AddrParseError { .. }
      | AppError::ToStrError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
      AppError::Authorization
      | AppError::PasswordError
      | AppError::InvalidJwtSignature
      | AppError::ExpiredSignature => StatusCode::UNAUTHORIZED,
      AppError::Forbidden => StatusCode::FORBIDDEN,
      AppError::UserNotFound
      | AppError::SiteNotFound
      | AppError::AgentNotFound
      | AppError::DeploymentNotFound => StatusCode::NOT_FOUND,
      AppError::UserExists | AppError::AgentExists => StatusCode::CONFLICT,
      AppError::Params { .. } => StatusCode::BAD_REQUEST,
      AppError::NotImplemented => StatusCode::NOT_IMPLEMENTED,
    }
  }

  pub fn user_message(&self) -> String {
    "user message".to_string()
  }
}

impl ResponseError for AppError {
  fn error_response(&self) -> HttpResponse {
    tracing::error!("{}", self.to_string());
    HttpResponse::build(self.status_code()).json(Response::<Option<()>> {
      data: None,
      code: self.code(),
      msg: self.to_string(), // 使用 thiserror 格式化的错误消息，或调用 user_message()
    })
  }
}

impl From<helpers::jwt::Error> for AppError {
  fn from(err: helpers::jwt::Error) -> Self {
    match err.kind() {
      helpers::jwt::ErrorKind::InvalidSignature => AppError::InvalidJwtSignature,
      helpers::jwt::ErrorKind::ExpiredSignature => AppError::ExpiredSignature,
      _ => AppError::JwtError { source: err },
    }
  }
}

// impl From<rpc::error::Error> for AppError {
//   fn from(err: rpc::error::Error) -> Self {
//     match err.kind() {
//       helpers::jwt::ErrorKind::InvalidSignature => AppError::InvalidJwtSignature,
//       helpers::jwt::ErrorKind::ExpiredSignature => AppError::ExpiredSignature,
//       _ => AppError::JwtError { source: err },
//     }
//   }
// }
