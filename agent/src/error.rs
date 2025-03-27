use actix_web::{HttpResponse, ResponseError, http::StatusCode};

#[derive(Debug)]
pub enum AppError {
  Error,
  Env,
  AuthFailed,
  Forbidden,
  ValidationFailed,
  DeploymentNotFound,
  RpcCallError,
}

impl AppError {
  pub fn code(&self) -> i32 {
    match self {
      AppError::Error => 1000,
      AppError::AuthFailed => todo!(),
      AppError::Forbidden => todo!(),
      AppError::ValidationFailed => todo!(),
      AppError::DeploymentNotFound => todo!(),
      AppError::Env => 1000,
      AppError::RpcCallError => 3000,
    }
  }
  pub fn message(&self) -> String {
    match self {
      AppError::Error => "error".to_string(),
      AppError::AuthFailed => "认证失败".to_string(),
      AppError::Forbidden => "".to_string(),
      AppError::ValidationFailed => "".to_string(),
      AppError::DeploymentNotFound => "".to_string(),
      AppError::Env => "".to_string(),
      AppError::RpcCallError => "".to_string(),
    }
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
