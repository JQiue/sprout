use actix_web::HttpResponse;
use common::Response;
use serde::Serialize;

use crate::{error::AppError, types::ServiceResult};

pub trait AgentResponse<T> {
  fn success(data: T) -> Result<HttpResponse, AppError>;
  fn error(error: AppError) -> Result<HttpResponse, AppError>;
}

impl<T> AgentResponse<T> for Response<T>
where
  T: Serialize,
{
  fn success(data: T) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(Response {
      data,
      code: 0,
      msg: "".to_string(),
    }))
  }

  fn error(error: AppError) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(Response::<Option<T>> {
      data: None,
      code: error.code(),
      msg: error.to_string(),
    }))
  }
}

pub trait IntoHttpResponse<T> {
  fn into_http_response(self) -> Result<HttpResponse, AppError>;
}

impl<T> IntoHttpResponse<T> for ServiceResult<T>
where
  T: Serialize,
{
  fn into_http_response(self) -> Result<HttpResponse, AppError> {
    match self {
      Ok(data) => Response::<T>::success(data),
      Err(err) => Response::<T>::error(err),
    }
  }
}
