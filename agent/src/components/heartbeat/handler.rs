use actix_web::{HttpResponse, get};

use crate::{components::heartbeat::service, error::AppError, response::Response};

#[get("/heartbeat")]
pub async fn heartbeat() -> Result<HttpResponse, AppError> {
  match service::heartbeat().await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}
