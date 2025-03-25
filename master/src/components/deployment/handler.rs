use actix_web::{
  get, post,
  web::{Data, Json, Path},
  HttpResponse,
};

use crate::{
  app::AppState, components::deployment::model::UpdateDeploymentStatusBody, response::Response,
};

use super::service;

#[get("/deployment/{deployment_id}")]
pub async fn get_deployment_info(state: Data<AppState>, deployment_id: Path<i32>) -> HttpResponse {
  match service::get_deployment_info(&state, deployment_id.into_inner()).await {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}

#[post("/deployment/status")]
pub async fn update_deployment_status(
  state: Data<AppState>,
  body: Json<UpdateDeploymentStatusBody>,
) -> HttpResponse {
  let Json(UpdateDeploymentStatusBody {
    agent_id,
    agent_token,
    deployment_id,
    status,
  }) = body;
  match service::update_deployment_status(agent_id, agent_token, deployment_id, status, &state.db)
    .await
  {
    Ok(data) => HttpResponse::Ok().json(Response::success(data)),
    Err(err) => HttpResponse::Ok().json(Response::<()>::error(err)),
  }
}
