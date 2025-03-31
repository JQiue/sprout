use entity::deployment::{self, DeploymentStatus};
use helpers::{jwt, time::utc_now};
use sea_orm::Set;
use serde_json::{Value, json};

use crate::{app::AppState, error::AppError};

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
  agent_id: u32,
  agent_token: String,
  deployment_id: u32,
  status: DeploymentStatus,
) -> Result<Value, AppError> {
  let agent = state
    .repo
    .agent()
    .get_agent(agent_id)
    .await?
    .ok_or(AppError::AgentNotFound)?;
  if agent.token != agent_token {
    return Err(AppError::AgentAuthFailed);
  }
  if !state
    .repo
    .deployment()
    .has_deployment(deployment_id)
    .await?
  {
    return Err(AppError::DeploymentNotFound);
  }
  jwt::verify::<String>(&agent_token, &state.register_agent_key)?;
  let deployment = state
    .repo
    .deployment()
    .update_deployment(deployment::ActiveModel {
      id: Set(deployment_id),
      status: Set(status),
      execution_time: Set(utc_now()),
      ..Default::default()
    })
    .await?;
  Ok(json!(deployment))
}
