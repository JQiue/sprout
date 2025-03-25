use actix_web::{
  get, post,
  web::{Data, Json, Path, ReqData},
  HttpResponse,
};
use serde_json::json;

use crate::{
  app::AppState,
  components::agent::{model::*, service},
  middleware::auth::JwtPayload,
  response::Response,
};

#[post("/agent")]
pub async fn register_agent(
  state: Data<AppState>,
  req_data: ReqData<JwtPayload>,
  body: Json<RegisterAgentBody>,
) -> HttpResponse {
  let Json(RegisterAgentBody {
    hostname,
    ip_address,
    storage_path,
    available_space,
    status,
  }) = body;
  match service::register_agent(
    &state,
    req_data.user_id.clone(),
    hostname,
    ip_address,
    storage_path,
    available_space,
    status,
  )
  .await
  {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}

#[get("/agent/{agent_id}")]
pub async fn get_agent_status(state: Data<AppState>, agent_id: Path<i32>) -> HttpResponse {
  match service::get_agent_status(&state, agent_id.into_inner()).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(json!({
      "cpu_cores" : data.data.cpu_cores,
      "cpu_usage" : data.data.cpu_usage,
      "total_memory": data.data.total_memory,
      "free_memory": data.data.free_memory,
      "memory_usage": data.data.memory_usage,
    }))),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}

#[post("/agent/{agent_id}/token")]
pub async fn refresh_agent_token(
  state: Data<AppState>,
  req_data: ReqData<JwtPayload>,
  agent_id: Path<i32>,
) -> HttpResponse {
  match service::refresh_agent_token(&state, req_data.user_id.clone(), agent_id.into_inner()).await
  {
    Ok(data) => HttpResponse::Ok().json(Response::success(json!(data))),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}
