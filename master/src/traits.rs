use actix_web::HttpResponse;
use common::Response;
use serde::Serialize;

use crate::{error::AppError, types::ServiceResult};

pub trait MasterResponse<T> {
  fn success(data: T) -> Result<HttpResponse, AppError>;
  fn error(error: AppError) -> Result<HttpResponse, AppError>;
}

impl<T> MasterResponse<T> for Response<T>
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
    Err(error)
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
