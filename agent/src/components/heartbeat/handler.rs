use actix_web::{HttpResponse, get, web::Data};

use crate::{app::AppState, components::heartbeat::service, error::AppError, response::Response};

#[get("/heartbeat")]
pub async fn heartbeat(state: Data<AppState>) -> Result<HttpResponse, AppError> {
  match service::heartbeat(&state).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}
