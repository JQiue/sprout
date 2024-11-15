use actix_web::{
  post,
  web::{Data, Json},
  HttpResponse,
};
use serde_json::json;

use crate::{
  app::AppState,
  components::agent::{model::*, service},
  response::{Response, StatusCode},
};

#[post("/agent")]
pub async fn register_agent(state: Data<AppState>, body: Json<RegisterAgentBody>) -> HttpResponse {
  let Json(RegisterAgentBody {
    hostname,
    ip_address,
    storage_path,
    available_space,
    status,
  }) = body;

  match service::register_agent(&state).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}
