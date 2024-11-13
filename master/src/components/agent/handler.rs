use actix_web::{
  post,
  web::{Data, Json},
  HttpResponse,
};
use serde_json::json;

use crate::{
  app::AppState,
  components::agent::{model::*, service},
};

#[post("/user")]
pub async fn register_server(
  state: Data<AppState>,
  body: Json<RegisterServerBody>,
) -> HttpResponse {
  let Json(RegisterServerBody {
    hostname,
    ip_address,
    storage_path,
    available_space,
    status,
  }) = body;

  match service::register_server(&state).await {
    Ok(data) => HttpResponse::Ok().json(json!({
     "data": data,
     "errmsg": "",
     "errno": 0,
    })),
    Err(err) => HttpResponse::Ok().json(json!({
     "errmsg": err,
     "errno": 1000,
    })),
  }
}
