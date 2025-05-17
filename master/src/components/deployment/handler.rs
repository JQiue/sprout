use actix_web::{
  HttpRequest, HttpResponse, get, post,
  web::{Data, Json, Path},
};
use common::master::CreateDeploymentRequest;

use crate::{
  app::AppState,
  components::deployment::model::{UpdateDeploymentRequest, UpdateDeploymentStatusBody},
  error::AppError,
  helper::extract_token,
  traits::IntoHttpResponse,
};

use super::service;

#[post("/deployment")]
pub async fn create_deployment(
  state: Data<AppState>,
  body: Json<CreateDeploymentRequest>,
) -> Result<HttpResponse, AppError> {
  service::create_deployment(&state, body.0.site_id)
    .await
    .into_http_response()
}

#[get("/deployment/{deployment_id}")]
pub async fn get_deployment(
  state: Data<AppState>,
  deployment_id: Path<u32>,
) -> Result<HttpResponse, AppError> {
  service::get_deployment_info(&state, deployment_id.into_inner())
    .await
    .into_http_response()
}

#[post("/deployment")]
pub async fn update_deployment(
  state: Data<AppState>,
  req: HttpRequest,
  body: Json<UpdateDeploymentRequest>,
) -> Result<HttpResponse, AppError> {
  let token = extract_token(&req)?;
  service::update_deployment(&state, token, body.0.deployment_id, body.0.status)
    .await
    .into_http_response()
}

#[post("/deployment/status")]
pub async fn update_deployment_status(
  state: Data<AppState>,
  body: Json<UpdateDeploymentStatusBody>,
) -> Result<HttpResponse, AppError> {
  let Json(UpdateDeploymentStatusBody {
    agent_token,
    deployment_id,
    status,
  }) = body;
  service::update_deployment_status(&state, agent_token, deployment_id, status)
    .await
    .into_http_response()
}
