use entity::deployment::{self, DeploymentStatus};
use helpers::{jwt, time::utc_now};
use sea_orm::{IntoActiveModel, Set};
use serde_json::{Value, json};

use crate::{app::AppState, error::AppError};

pub async fn create_deployment(state: &AppState, site_id: String) -> Result<Value, AppError> {
  if !state.repo.site().has_site(&site_id).await? {
    return Err(AppError::SiteNotFound);
  }
  if let Some(agent) = state.repo.agent().get_avaliable_agent().await? {
    let deployment = state
      .repo
      .deployment()
      .create_deployment(deployment::ActiveModel {
        status: Set(DeploymentStatus::Pending),
        agent_id: Set(agent.id),
        site_id: Set(site_id.clone()),
        execution_time: Set(utc_now()),
        created_at: Set(utc_now()),
        ..Default::default()
      })
      .await?;
    let agent_rpc = rpc::Agent::Rpc::new();
    let init_response = agent_rpc
      .init_upload_session(&agent.ip_address, &deployment.site_id, deployment.id)
      .await?;
    let mut active_deployment = deployment.into_active_model();
    active_deployment.status = Set(DeploymentStatus::Uploading);
    active_deployment.upload_token = Set(Some(init_response.upload_token.clone()));
    let deployment = state
      .repo
      .deployment()
      .update_deployment(active_deployment)
      .await?;
    Ok(json!({
        "upload_url": agent.ip_address,
        "upload_token": init_response.upload_token,
        "site_id": site_id,
        "agent_id": deployment.agent_id,
        "deployment_id": deployment.id,
    }))
  } else {
    Err(AppError::AgentNotFound)
  }
}

pub async fn get_deployment_info(state: &AppState, deployment_id: u32) -> Result<Value, AppError> {
  if let Some(deployment) = state
    .repo
    .deployment()
    .get_deployment(deployment_id)
    .await?
  {
    Ok(json!(deployment))
  } else {
    Err(AppError::DeploymentNotFound)
  }
}

pub async fn update_deployment_status(
  state: &AppState,
  agent_token: String,
  deployment_id: u32,
  status: DeploymentStatus,
) -> Result<Value, AppError> {
  jwt::verify::<String>(&agent_token, &state.register_agent_key)?;
  state
    .repo
    .deployment()
    .update_deployment(deployment::ActiveModel {
      id: Set(deployment_id),
      status: Set(status),
      execution_time: Set(utc_now()),
      ..Default::default()
    })
    .await?;
  Ok(json!(()))
}
