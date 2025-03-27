use std::fmt::{Debug, Display};

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
  pub code: i32,
  pub msg: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
}

impl<T> Response<T>
where
  T: Serialize,
{
  pub fn success(data: Option<T>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(Response {
      data,
      code: 0,
      msg: "".to_string(),
    }))
  }

  pub fn error(error: AppError) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(Response::<()> {
      data: None,
      code: error.code(),
      msg: error.message(),
    }))
  }
}

impl<T> Display for Response<T>
where
  T: Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, r#"{{ "code": {}, "msg": "{}" }}"#, self.code, self.msg)
  }
}
