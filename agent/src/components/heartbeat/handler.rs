use actix_web::{get, web::Data, HttpResponse};

use crate::{app::AppState, components::heartbeat::service, response::Response};

#[get("/heartbeat")]
pub async fn heartbeat(state: Data<AppState>) -> HttpResponse {
  match service::heartbeat(&state).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}
