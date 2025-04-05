use actix_web::{
  HttpResponse, get, post,
  web::{Data, Json, Path},
};

use crate::{
  app::AppState,
  components::deployment::model::{CreateDeploymentBody, UpdateDeploymentStatusBody},
  error::AppError,
  response::Response,
};

use super::service;

#[post("/deployment")]
pub async fn create_deployment(
  state: Data<AppState>,
  body: Json<CreateDeploymentBody>,
) -> Result<HttpResponse, AppError> {
  match service::create_deployment(&state, body.0.site_id).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[get("/deployment/{deployment_id}")]
pub async fn get_deployment_info(
  state: Data<AppState>,
  deployment_id: Path<u32>,
) -> Result<HttpResponse, AppError> {
  match service::get_deployment_info(&state, deployment_id.into_inner()).await {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}

#[post("/deployment/status")]
pub async fn update_deployment_status(
  state: Data<AppState>,
  body: Json<UpdateDeploymentStatusBody>,
) -> Result<HttpResponse, AppError> {
  let Json(UpdateDeploymentStatusBody {
    agent_id,
    agent_token,
    deployment_id,
    status,
  }) = body;
  match service::update_deployment_status(&state, agent_id, agent_token, deployment_id, status)
    .await
  {
    Ok(data) => Response::success(Some(data)),
    Err(err) => Response::<()>::error(err),
  }
}
