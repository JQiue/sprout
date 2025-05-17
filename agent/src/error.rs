use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use common::Response;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error("Load environment variable error")]
  LoadEnv {
    #[from]
    source: dotenvy::Error,
  },
  #[error("Deserializes environment variable error")]
  DeserializeEnv {
    #[from]
    source: envy::Error,
  },
  #[error("Temp file not found error")]
  TempfileNotFound,
  #[error("Extract tar error")]
  ExtractTar,
  #[error("Nginx deploy error")]
  NginxDeploy,
  #[error("Internal server error {source:?}")]
  InternalServerError {
    #[source]
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
  },
}

impl AppError {
  pub fn code(&self) -> i32 {
    match self {
      AppError::InternalServerError { .. }
      | AppError::LoadEnv { .. }
      | AppError::DeserializeEnv { .. }
      | AppError::TempfileNotFound
      | AppError::ExtractTar
      | AppError::NginxDeploy => 1000,
    }
  }

  pub fn status_code(&self) -> StatusCode {
    match self {
      AppError::InternalServerError { .. }
      | AppError::LoadEnv { .. }
      | AppError::DeserializeEnv { .. }
      | AppError::TempfileNotFound
      | AppError::ExtractTar
      | AppError::NginxDeploy => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

impl ResponseError for AppError {
  fn error_response(&self) -> HttpResponse {
    tracing::error!("{:#?}", self);
    HttpResponse::build(self.status_code()).json(Response::<Option<()>> {
      data: None,
      code: self.code(),
      msg: self.to_string(), // 使用 thiserror 格式化的错误消息，或调用 user_message()
    })
  }
}

impl From<std::io::Error> for AppError {
  fn from(err: std::io::Error) -> Self {
    AppError::InternalServerError {
      source: Some(Box::new(err)),
    }
  }
}

impl From<helpers::jwt::Error> for AppError {
  fn from(err: helpers::jwt::Error) -> Self {
    AppError::InternalServerError {
      source: Some(Box::new(err)),
    }
  }
}

impl From<actix_web::http::header::ToStrError> for AppError {
  fn from(err: actix_web::http::header::ToStrError) -> Self {
    AppError::InternalServerError {
      source: Some(Box::new(err)),
    }
  }
}
