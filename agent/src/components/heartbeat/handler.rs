use crate::{components::heartbeat::service, error::AppError, traits::IntoHttpResponse};
use actix_web::{HttpResponse, get};

#[get("/heartbeat")]
pub async fn heartbeat() -> Result<HttpResponse, AppError> {
  service::heartbeat().await.into_http_response()
}
